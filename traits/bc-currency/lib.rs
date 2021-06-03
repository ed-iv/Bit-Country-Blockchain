#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_runtime::{
    traits::{AtLeast32Bit, MaybeSerializeDeserialize},
    DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::vec::Vec;
use primitives::{Balance, CountryId, CurrencyId};
use frame_support::traits::ExistenceRequirement;

// Stick in types

// Interface
pub trait BCCurrency {
    // Continuum
    // fn free_balance(who: AccountId);
    
    // fn transfer(
    //     from: AccountId, 
    //     to: AccountId, 
    //     amount: Balance,
    //     existence_requirement: ExistenceRequirement
    // );

}