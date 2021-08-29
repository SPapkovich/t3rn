// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use substrate_test_runtime_client::{self, DefaultTestClientBuilderExt};

use sp_keystore::KeystoreExt;

use sp_io::TestExternalities;

use hex_literal::hex;
use sc_rpc::author::AuthorApi;
use sc_rpc::state::StateApi;
use sc_rpc::system::SystemApi;

use t3rn_primitives::*;

use pallet_circuit_execution_delivery::message_assembly::test_utils::create_test_stuffed_gateway_protocol;

use sp_keyring::Sr25519Keyring;

use crate::mock_rpc_setup::{uxt, TestSetup};

#[test]
fn rpc_prints_system_version() {
    let p = TestSetup::default();

    let mut io = jsonrpc_core::MetaIoHandler::<sc_rpc::Metadata>::default();

    io.extend_with(AuthorApi::to_delegate(p.author()));
    io.extend_with(SystemApi::to_delegate(p.system()));
    io.extend_with(StateApi::to_delegate(p.state()));

    let request = r#"{"jsonrpc":"2.0","method":"system_version","params":[],"id":1}"#;

    let response = r#"{"jsonrpc":"2.0","result":"0.1.0","id":1}"#;

    let meta = sc_rpc::Metadata::default();
    assert_eq!(io.handle_request_sync(request, meta), Some(response.into()));
}

#[test]
fn successfully_dispatches_unsigned_get_storage_outbound_message_from_circuit_to_external_gateway()
{
    let p = TestSetup::default();

    let mut io = jsonrpc_core::MetaIoHandler::<sc_rpc::Metadata>::default();

    io.extend_with(AuthorApi::to_delegate(p.author()));
    io.extend_with(SystemApi::to_delegate(p.system()));
    io.extend_with(StateApi::to_delegate(p.state()));

    let key: Vec<u8> = hex!("0befda6e1ca4ef40219d588a727f1271").to_vec();

    let expected_storage = GatewayExpectedOutput::Storage {
        key: vec![key.clone()],
        value: vec![None],
    };

    let arguments = vec![key];

    let get_storage_outbound_message: CircuitOutboundMessage = CircuitOutboundMessage {
        name: b"get_storage".to_vec(),
        module_name: b"state".to_vec(),
        method_name: b"getStorage".to_vec(),
        arguments,
        expected_output: vec![expected_storage],
        extra_payload: None,
        sender: None,
        target: None,
    };

    let request_message: RpcPayloadUnsigned =
        get_storage_outbound_message.to_jsonrpc_unsigned().unwrap();

    let request = format!(
        r#"{{"jsonrpc":"2.0","method":"state_getStorage","params":["0x{}"],"id":1}}"#,
        // request_message.method_name,
        hex::encode(request_message.params.get(0).unwrap())
    );
    // let response = r#"{"jsonrpc":"2.0","result":"0x03fd9651c5ffb80b68eb4faddc697b50016128f8e3799ff81ae42d78d38ba9e4","id":1}"#;
    let response = r#"{"jsonrpc":"2.0","result":"0xf1d130ad6d72ddd7a01d6bae3f291a91956ddd938c3fdbf0c1df2186ab4a014c","id":1}"#;

    let meta = sc_rpc::Metadata::default();
    assert_eq!(
        io.handle_request_sync(&request, meta),
        Some(response.into())
    );
}

#[test]
fn successfully_dispatches_signed_transfer_outbound_message_from_circuit_to_external_gateway() {
    let p = TestSetup::default();

    let mut io = jsonrpc_core::MetaIoHandler::<sc_rpc::Metadata>::default();

    io.extend_with(AuthorApi::to_delegate(p.author()));
    io.extend_with(SystemApi::to_delegate(p.system()));
    io.extend_with(StateApi::to_delegate(p.state()));

    let test_protocol = create_test_stuffed_gateway_protocol(Sr25519Keyring::Alice.public().into());

    let mut ext = TestExternalities::new_empty();
    ext.register_extension(KeystoreExt(p.keystore));
    ext.execute_with(|| {
        let _transfer_message = test_protocol
            .transfer(
                Default::default(),
                Default::default(),
                GatewayType::ProgrammableExternal,
            )
            .unwrap();

        let transfer = uxt(Sr25519Keyring::Alice, 0);

        use sp_core::Encode;
        let request = format!(
            r#"{{"jsonrpc":"2.0","method":"author_submitExtrinsic","params":["0x{}"],"id":1}}"#,
            hex::encode(transfer.encode())
        );

        let response = r#"{"jsonrpc":"2.0","result":null,"id":1}"#;

        let meta = sc_rpc::Metadata::default();
        assert_eq!(
            io.handle_request_sync(&request, meta),
            Some(response.into())
        );
    });
}
