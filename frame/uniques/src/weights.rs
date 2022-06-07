// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_uniques
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-03, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_uniques
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/uniques/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_uniques.
pub trait WeightInfo {
	fn create() -> Weight;
	fn force_create() -> Weight;
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight;
	fn mint() -> Weight;
	fn burn() -> Weight;
	fn transfer() -> Weight;
	fn redeposit(i: u32, ) -> Weight;
	fn freeze() -> Weight;
	fn thaw() -> Weight;
	fn freeze_collection() -> Weight;
	fn thaw_collection() -> Weight;
	fn transfer_ownership() -> Weight;
	fn set_team() -> Weight;
	fn force_item_status() -> Weight;
	fn set_attribute() -> Weight;
	fn clear_attribute() -> Weight;
	fn set_metadata() -> Weight;
	fn clear_metadata() -> Weight;
	fn set_collection_metadata() -> Weight;
	fn clear_collection_metadata() -> Weight;
	fn approve_transfer() -> Weight;
	fn cancel_approval() -> Weight;
	fn set_accept_ownership() -> Weight;
	fn set_collection_max_supply() -> Weight;
	fn set_price() -> Weight;
	fn buy_item() -> Weight;
}

/// Weights for pallet_uniques using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn create() -> Weight {
		(27_715_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_create() -> Weight {
		(16_929_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Attribute (r:0 w:1000)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques InstanceMetadataOf (r:0 w:1000)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Storage: Uniques Account (r:0 w:20)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((10_481_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 16_000
			.saturating_add((1_762_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 16_000
			.saturating_add((1_590_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques Account (r:0 w:1)
	fn mint() -> Weight {
		(36_577_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	fn burn() -> Weight {
		(36_124_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:2)
	fn transfer() -> Weight {
		(27_225_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:100 w:100)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 14_000
			.saturating_add((12_407_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn freeze() -> Weight {
		(21_452_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn thaw() -> Weight {
		(22_155_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn freeze_collection() -> Weight {
		(16_897_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn thaw_collection() -> Weight {
		(16_657_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:2)
	fn transfer_ownership() -> Weight {
		(25_057_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn set_team() -> Weight {
		(17_253_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_item_status() -> Weight {
		(20_010_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		(41_159_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn clear_attribute() -> Weight {
		(39_598_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		(33_387_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		(33_208_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn set_collection_metadata() -> Weight {
		(32_619_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn clear_collection_metadata() -> Weight {
		(31_028_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn approve_transfer() -> Weight {
		(22_263_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn cancel_approval() -> Weight {
		(22_910_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	fn set_accept_ownership() -> Weight {
		(20_716_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn set_collection_max_supply() -> Weight {
		(19_380_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn set_price() -> Weight {
		(19_412_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn buy_item() -> Weight {
		(19_412_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn create() -> Weight {
		(27_715_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_create() -> Weight {
		(16_929_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Attribute (r:0 w:1000)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques InstanceMetadataOf (r:0 w:1000)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Storage: Uniques Account (r:0 w:20)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((10_481_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 16_000
			.saturating_add((1_762_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 16_000
			.saturating_add((1_590_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques Account (r:0 w:1)
	fn mint() -> Weight {
		(36_577_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	fn burn() -> Weight {
		(36_124_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:2)
	fn transfer() -> Weight {
		(27_225_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:100 w:100)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 14_000
			.saturating_add((12_407_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn freeze() -> Weight {
		(21_452_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn thaw() -> Weight {
		(22_155_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn freeze_collection() -> Weight {
		(16_897_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn thaw_collection() -> Weight {
		(16_657_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:2)
	fn transfer_ownership() -> Weight {
		(25_057_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn set_team() -> Weight {
		(17_253_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_item_status() -> Weight {
		(20_010_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		(41_159_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn clear_attribute() -> Weight {
		(39_598_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		(33_387_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		(33_208_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn set_collection_metadata() -> Weight {
		(32_619_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn clear_collection_metadata() -> Weight {
		(31_028_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn approve_transfer() -> Weight {
		(22_263_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn cancel_approval() -> Weight {
		(22_910_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	fn set_accept_ownership() -> Weight {
		(20_716_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn set_collection_max_supply() -> Weight {
		(19_380_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn set_price() -> Weight {
		(19_412_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn buy_item() -> Weight {
		(19_412_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
