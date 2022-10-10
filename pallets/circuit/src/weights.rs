//! Autogenerated weights for pallet_circuit_circuit_portal
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-09-24, STEPS: `[50, ]`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/circuit
// benchmark
// --chain=dev
// --steps=50
// --repeat=100
// --pallet=pallet_circuit
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./src/circuit/src/weights.rs
// --template=../benchmarking/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_circuit_circuit_portal.
pub trait WeightInfo {
    fn on_local_trigger() -> Weight;
    fn on_extrinsic_trigger() -> Weight;
    fn bid_execution() -> Weight;
    fn confirm_side_effect() -> Weight;
    fn execute_side_effects_with_xbi() -> Weight;
}

/// Weights for pallet_circuit_circuit_portal using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn on_local_trigger() -> Weight {
        6_984_000_u64
    }

    fn on_extrinsic_trigger() -> Weight {
        60_000_000_u64
    }

    fn confirm_side_effect() -> Weight {
        60_000_000_u64
    }

    fn bid_execution() -> Weight {
        60_000_000_u64
    }

    fn execute_side_effects_with_xbi() -> Weight {
        60_000_000_u64
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn on_local_trigger() -> Weight {
        6_984_000_u64
    }

    fn on_extrinsic_trigger() -> Weight {
        60_000_000_u64
    }

    fn confirm_side_effect() -> Weight {
        60_000_000_u64
    }

    fn bid_execution() -> Weight {
        60_000_000_u64
    }

    fn execute_side_effects_with_xbi() -> Weight {
        60_000_000_u64
    }
}
