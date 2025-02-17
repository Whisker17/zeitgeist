//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=frame_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for frame_system (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::weights::WeightInfo for WeightInfo<T> {
    fn remark(_b: u32) -> Weight {
        (8_454_000 as Weight)
    }
    fn remark_with_event(b: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
    }
    // Storage: System Digest (r:1 w:1)
    // Storage: unknown [0x3a686561707061676573] (r:0 w:1)
    fn set_heap_pages() -> Weight {
        (6_950_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn set_storage(i: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 16_000
            .saturating_add((1_090_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn kill_storage(i: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 8_000
            .saturating_add((801_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn kill_prefix(p: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 12_000
            .saturating_add((1_703_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
    }
}
