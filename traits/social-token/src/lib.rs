#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_runtime::{DispatchResult, DispatchError};
use primitives::{Balance, CountryId, Balance};

pub trait SocialToken<AccountId, CurrencyId> {

    fn minimum_balance(currency_id: CurrencyId) -> Balance;

    fn total_issuance(currency_id: CurrencyId) -> Balance;

    fn total_balance(currency_id: CurrencyId, who: &AccountId) -> Balance;

    fn free_balance(currency_id: CurrencyId, who: &AccountId) -> Balance;

    fn ensure_can_withdraw(currency_id: CurrencyId, who: &AccountId, amount: Balance) -> DispatchResult;

    fn transfer(
        currency_id: CurrencyId,
        from: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> DispatchResult;

    fn deposit(currency_id: CurrencyId, who: &AccountId, amount: Balance) -> DispatchResult;

    fn withdraw(currency_id: CurrencyId, who: &AccountId, amount: Balance) -> DispatchResult;

    fn can_slash(currency_id: CurrencyId, who: &AccountId, amount: Balance) -> bool;

    fn slash(currency_id: CurrencyId, who: &AccountId, amount: Balance) -> Balance;

    // MultiCurrencyExtended:
    fn update_balance(currency_id: CurrencyId, who: &AccountId, by_amount: Balance) -> DispatchResult;



    // fn deposit(currency_id: CurrencyId, who: &AccountId, amount: Balance) -> DispatchResult;


    // fn get_total_issuance(currency_id: CurrencyId) -> Balance;
    
    // fn get_total_issuance_by_country_id(country_id: CountryId) -> Result<Balance, DispatchError>;

    // fn transfer_from(
    //     currency_id: CurrencyId, 
    //     from: &AccountId, 
    //     to: &AccountId, 
    //     amount: Balance
    // ) -> DispatchResult;
    
    // fn deposit_to(
    //     currency_id: CurrencyId, 
    //     who: &AccountId, 
    //     amount: Balance
    // ) -> DispatchResult;

    // fn withdraw_from(
    //     currency_id: CurrencyId, 
    //     who: &AccountId, 
    //     amount: Balance
    // ) -> DispatchResult;
}
