// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Storage migrations for the vesting pallet.

use super::*;
use frame_support::traits::OnRuntimeUpgrade;
// Migration from single schedule to multiple schedules.
pub mod v1 {
	use super::*;

	#[cfg(feature = "try-runtime")]
	pub fn pre_migrate<T: Config>() -> Result<(), &'static str> {
		assert!(StorageVersion::<T>::get() == Releases::V0, "Storage version too high.");

		log::debug!(
			target: "runtime::vesting",
			"migration: Vesting storage version v1 PRE migration checks succesful!"
		);

		Ok(())
	}

	/// Manually setting the version to V1
	pub struct ForceSetVersionToV1<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for ForceSetVersionToV1<T> {
		fn on_runtime_upgrade() -> Weight {

			if StorageVersion::<T>::get() == Releases::V0 {
				StorageVersion::<T>::put(Releases::V1);
			}
			
			T::DbWeight::get().reads_writes(1, 1)
		}
	
	}

	#[cfg(feature = "try-runtime")]
	pub fn post_migrate<T: Config>() -> Result<(), &'static str> {
		assert_eq!(StorageVersion::<T>::get(), Releases::V1);

		for (_key, schedules) in Vesting::<T>::iter() {
			assert!(
				schedules.len() >= 1,
				"A bounded vec with incorrect count of items was created."
			);

			for s in schedules {
				// It is ok if this does not pass, but ideally pre-existing schedules would pass
				// this validation logic so we can be more confident about edge cases.
				if !s.is_valid() {
					log::warn!(
						target: "runtime::vesting",
						"migration: A schedule does not pass new validation logic.",
					)
				}
			}
		}

		log::debug!(
			target: "runtime::vesting",
			"migration: Vesting storage version v1 POST migration checks successful!"
		);
		Ok(())
	}
}
