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

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event, *};
use sp_core::blake2_256;
use sp_runtime::traits::BadOrigin;

fn check_balances(
    balances: Vec<(&AccountId, SocialTokenCurrencyId, mock::Balance)>) -> bool {
    for (who, currency_id, balance) in balances {        
        assert_eq!(SocialCurrencies::total_balance(currency_id, &who), balance);        
    }
    true
}

#[test]
fn add_liquidity_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {        
        // As written code requires pair begins w/ native token (NUUM)
        assert_noop!(
            SwapModule::add_liquidity(ALICE.into(), SOC_A, SOC_B, 10, 10),
            Error::<Runtime>::InvalidSocialTokenIds
        );
        // NUUM balance too low
        assert_noop!(
            SwapModule::add_liquidity(ALICE.into(), NUUM, SOC_A, 1000, 10),
            pallet_balances::Error::<Runtime>::InsufficientBalance
        );
        // SOC_A balance too low
        assert_noop!(
            SwapModule::add_liquidity(ALICE.into(), NUUM, SOC_A, 10, 1000),
            orml_tokens::Error::<Runtime>::BalanceTooLow
        );
        // Below existential deposit
        assert_noop!(
            SwapModule::add_liquidity(ALICE.into(), NUUM, SOC_A, 100, 10),
            pallet_balances::Error::<Runtime>::KeepAlive
        );
        // Invalid Liquidity Increment
        assert_noop!(
            SwapModule::add_liquidity(ALICE.into(), NUUM, SOC_A, 0, 10),
            Error::<Runtime>::InvalidLiquidityIncrement
        );
        assert_noop!(
            SwapModule::add_liquidity(ALICE.into(), NUUM, SOC_A, 10, 0),
            Error::<Runtime>::InvalidLiquidityIncrement
        );
        assert_noop!(
            SwapModule::add_liquidity(ALICE.into(), NUUM, SOC_A, 0, 0),
            Error::<Runtime>::InvalidLiquidityIncrement
        );

        // Pair not enabled
        // let status = TradingPairStatus::NotEnabled; 
        // let pair = TradingPair::from_token_currency_ids(NUUM, SOC_A);
        // TradingPairStatuses::<Runtime>::insert(pair, status);
        // assert_noop!(
        //     SwapModule::add_liquidity(ALICE.into(), NUUM, SOC_A, 10, 10),
        //     Error::<Runtime>::NotEnabledTradingPair
        // );
    });
}

#[test]
fn add_liquidity_should_work() {
    ExtBuilder::default().build().execute_with(|| {    
        let share_currency_id: SocialTokenCurrencyId = SocialTokenCurrencyId::DEXShare(0, 1);
        let dex_account_id: AccountId = SwapModule::account_id();
        let pair = TradingPair (NUUM, SOC_A);

        assert_ok!(SwapModule::add_liquidity(ALICE.into(), pair.0, pair.1, 10, 5));
    
        check_balances(vec![
            (&ALICE, pair.0, 90), 
            (&ALICE, pair.1, 95),
            (&ALICE, share_currency_id, 20),
            (&dex_account_id, pair.0, 10),
            (&dex_account_id, pair.1, 5),
        ]);        
    
        assert_eq!(SwapModule::liquidity_pool(pair), (10, 5));
        let event = mock::Event::swap(crate::Event::AddLiquidity(ALICE, pair.0, 10, pair.1, 5, 20));
        assert_eq!(last_event(), event);    
    });
}

#[test]
fn add_liquidity_should_work_with_less_nuum() {
    ExtBuilder::default().build().execute_with(|| {    
        assert_ok!(SwapModule::add_liquidity(ALICE.into(), NUUM, SOC_A, 5, 10));

        let share_currency_id: SocialTokenCurrencyId = SocialTokenCurrencyId::DEXShare(0, 1);
        let dex_account_id: AccountId = SwapModule::account_id();
        
        // Check ALICE's balances
        assert_eq!(SocialCurrencies::total_balance(NUUM, &ALICE), 95);
        assert_eq!(SocialCurrencies::total_balance(SOC_A, &ALICE), 90);
        assert_eq!(SocialCurrencies::total_balance(share_currency_id, &ALICE), 20);
        
        // Check pool balances:
        assert_eq!(SocialCurrencies::total_balance(NUUM, &dex_account_id), 5);
        assert_eq!(SocialCurrencies::total_balance(SOC_A, &dex_account_id), 10);
        
        let event = mock::Event::swap(crate::Event::AddLiquidity(ALICE, NUUM, 5, SOC_A, 10, 20));
        assert_eq!(last_event(), event);
    });
}