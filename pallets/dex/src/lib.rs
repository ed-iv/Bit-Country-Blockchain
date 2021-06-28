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
use pallet_tokenization::{CountryTreasury};


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

/// Status for TradingPair
#[derive(Clone, Copy, Encode, Decode, RuntimeDebug, PartialEq, Eq)]
pub enum TokenStatus {	
	NotEnabled,
	Enabled,
}

impl Default for TokenStatus {
    fn default() -> Self {
        Self::NotEnabled
    }
}


#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_tokenization::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type ModuleId: Get<ModuleId>;
    }

    /// Liquidity pool for SocialToken.
	///
	/// LiquidityPool: map SocialToken => (Balance, Balance (NUUM))
	#[pallet::storage]
	#[pallet::getter(fn liquidity_pool)]
	pub type LiquidityPool<T: Config> = 
        StorageMap<_, Twox64Concat, CurrencyId, (Balance, Balance), ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn token_statuses)]
    pub type TokenStatuses<T: Config> =
            StorageMap<_, Twox64Concat, CurrencyId, TokenStatus, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    // #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::metadata()]
    pub enum Event<T: Config> {
        TokenTradingEnabled(CurrencyId)
    }

    #[pallet::error]
    pub enum Error<T> {        
        TokenNotFound, 
        TradingAlreadyEnabled,
        NoPermission,  
        CountryFundIsNotAvailable,  
        SocialTokenNotAvailable, 
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {        
        #[pallet::weight(10_000)]
        pub(super) fn enable_trading(
            origin: OriginFor<T>, 
            country_id: CountryId            
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            
            // Get Fund
            let country_fund = CountryTreasury::<T>::get(&country_id)
                .ok_or(Error::<T>::CountryFundIsNotAvailable)?;

            // Make sure who has permission to enable trading for country currency
            ensure!(
                T::CountryInfoSource::check_ownership(&who, &country_id), 
                Error::<T>::NoPermission
            );            
                            
            // Make sure not enabled
            ensure!(
				matches!(
					Self::token_statuses(country_fund.currency_id),
					TokenStatus::NotEnabled
				),
				Error::<T>::TradingAlreadyEnabled
			);

            // Light it up
            TokenStatuses::<T>::mutate(country_fund.currency_id, TokenStatus::Enabled);
			Self::deposit_event(Event::TokenTradingEnabled(country_fund.currency_id));
			
            Ok(().into())
        }   
        

        #[pallet::weight(10_000)]
        pub fn add_liquidity(
			origin: OriginFor<T>,
			currency_id_a: CurrencyId,
			currency_id_b: CurrencyId,
			// #[pallet::compact] max_amount_a: Balance,
			// #[pallet::compact] max_amount_b: Balance,
			// #[pallet::compact] min_share_increment: Balance,
			deposit_increment_share: bool,
		) -> DispatchResultWithPostInfo {
            
            // let country_token = SocialTokenModule::<T>::SocialTokens.get(country_fund.currency_id)
            //     .ok_or(Error::<T>::SocialTokenNotAvailable)?;
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

    impl<T: Config> Module<T> {    
        fn some_function(owner: &T::AccountId, metadata: Vec<u8>) -> Result<(), DispatchError> {
            Ok(())
        }
    }
}

