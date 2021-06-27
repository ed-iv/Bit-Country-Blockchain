// This file is part of Bit.Country.

// Copyright (C) 2020-2021 Bit.Country.
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

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::ensure;
use frame_system::{ensure_root, ensure_signed};
use primitives::{Balance, CountryId, CurrencyId};
use sp_runtime::{traits::{AccountIdConversion, One}, DispatchError, ModuleId, RuntimeDebug};
use bc_country::*;
use sp_std::vec::Vec;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type ModuleId: Get<ModuleId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn next_country_id)]
    pub type NextCountryId<T: Config> = StorageValue<_, CountryId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_country)]
    pub type Countries<T: Config> =
    StorageMap<_, Twox64Concat, CountryId, Country<T::AccountId>, OptionQuery>;


    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    // #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::metadata()]
    pub enum Event<T: Config> {
        // NewCountryCreated(CountryId),        
    }

    #[pallet::error]
    pub enum Error<T> {        
        TokenPairNotFound,        
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {        
        #[pallet::weight(10_000)]
        pub(super) fn enable_trading(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            Ok(().into())
        }   

        #[pallet::weight(10_000)]
        pub(super) fn add_liquidity(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            Ok(().into())
        }   

        #[pallet::weight(10_000)]
        pub(super) fn remove_liquidity(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            Ok(().into())
        }
        
        #[pallet::weight(10_000)]
        pub(super) fn swap(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            Ok(().into())
        }
        
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}
}

impl<T: Config> Module<T> {    
    fn some_function(owner: &T::AccountId, metadata: Vec<u8>) -> Result<_, DispatchError> {
        Ok()
    }
}