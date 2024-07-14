use bigdecimal::{BigDecimal, ToPrimitive, Zero, num_bigint::Sign};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use rand_distr::{Distribution};

use near_sdk::*;
use near_sdk::json_types::U128;
use near_sdk::collections::{LookupMap, TreeMap, Vector};

mod dex;
use dex::*;

mod token;
use token::*;

#[near(contract_state)]
pub struct Contract {
    dex_id: AccountId,
    arms: Vector<BigDeDecimal>,
    rewards: TreeMap<BigDeDecimal, (BigDeDecimal, BigDeDecimal)>,
    mean_gain: TreeMap<BigDeDecimal, BigDeDecimal>,
    inventory: LookupMap<Token, BigDeDecimal>,
    trade_size: BigDeDecimal,
}

pub const REF_FI_ACCOUNT_ID: &str = "ref-finance-101.testnet";

impl Default for Contract {
    fn default() -> Self {
        let dex_id = AccountId::from_str(REF_FI_ACCOUNT_ID).unwrap();

        let start = BigDecimal::from_str("0.0001").unwrap();
        let end = BigDecimal::from_str("0.03").unwrap();
        let num_elements: u32 = 50;

        let step = (end - &start) / BigDecimal::from(num_elements - 1);
        let trade_size = BigDecimal::from_str("0.1").unwrap().into();

        let mut arms = Vector::new(b"arms".to_vec());
        let mut rewards = TreeMap::new(b"rewards".to_vec());
        let mut mean_gain = TreeMap::new(b"mean_gain".to_vec());
        let mut inventory = LookupMap::new(b"inventory".to_vec());

        // Create initial state
        inventory.insert(&Token::near(), &BigDecimal::from(200).into());

        let bins = (0..num_elements).map(|i| &start + &step * BigDecimal::from(i.to_u32().unwrap()));
        for bin in bins {
            let arm = bin.into();
            let zero = || BigDecimal::zero().into();
            arms.push(&arm);
            mean_gain.insert(&arm, &zero());
            rewards.insert(&arm, &(zero(), zero()));
        }

        Self {
            dex_id,
            arms,
            rewards,
            mean_gain,
            inventory,
            trade_size,
        }
    }
}

#[near]
impl Contract {
    pub fn step(&mut self) -> Promise {
        let dex = dex::ref_fi::ext(self.dex_id.clone());
        let promise = dex.swap(vec![SwapAction {
            pool_id: NEAR_USDT_POOL_ID,
            token_in: AccountId::from_str(NEAR_TOKEN_ID).unwrap(),
            amount_in: Some(U128(10u128.pow(24))),
            token_out: AccountId::from_str(USDT_TOKEN_ID).unwrap(),
            min_amount_out: U128(5 * 10u128.pow(6)),
        }], None);

        return promise.then(
            // Create a promise to callback query_greeting_callback
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(5))
                .query_greeting_callback(
                    // AccountId::from_str(NEAR_TOKEN_ID).unwrap(),
                    // Some(U128(10u128.pow(24))),
                ),
        );
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn query_greeting_callback(
        &self,
        // token_in: AccountId,
        // amount_in: Option<U128>,
        #[callback_result] call_result: Result<U128, near_sdk::PromiseError>,
    ) -> String {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            log!("There was an error contacting RefFi");
            return "".to_string();
        }

        "HelloWorld".to_string()
    }


    // Takes as input a percentage gain obtained by a 3-hop swap starting and ending in NEAR
    // denominated in NEAR
    pub fn tick(&mut self, gain: BigDeDecimal) {
        let gain = BigDecimal::from(gain);
        let arm = self.get_arm(gain.clone());
        let expected_gain: BigDecimal = self.mean_gain.get(&arm.into()).unwrap().into();

        let near = &Token::near();

        let trade_size = self.trade_size.clone();
        let trade_size = BigDecimal::from(trade_size);

        let near_owned = self.inventory.get(near).unwrap();
        let near_owned = &BigDecimal::from(near_owned);

        let trained = self.rewards.len() >= 111;
        let profit_expected = expected_gain.to_f64().unwrap() > 0.0;

        if !trained || profit_expected {
            let pnl = self.trade(near_owned.mul(trade_size), gain.clone());
            let near_owned = near_owned.add(pnl);
            self.inventory.insert(near, &near_owned.into());
        }
    }

    fn get_arm(&mut self, gain: BigDecimal) -> BigDecimal {
        let arms = self.arms.iter().map(|arm| BigDecimal::from(arm.to_owned()))
            .collect::<Vec<BigDecimal>>();
        let last = arms.last().unwrap();
        let found = arms.iter().find(|x| **x >= gain);
        found.unwrap_or(last).to_owned()
    }
    fn get_gain_potential(&self) -> Option<BigDecimal>{
        let dex = dex::ref_fi::ext(self.dex_id.clone());
        // NEAR-USDT
        let amount_in_near_usdt = Some(U128(10u128.pow(24)));
        let promise_near_usdt = dex.get_return(
            NEAR_USDT_POOL_ID,
            AccountId::from_str(NEAR_TOKEN_ID).unwrap(),
            amount_in_near_usdt,
            AccountId::from_str(USDT_TOKEN_ID).unwrap()
        );
        let amount_out_near_usdt = promise_near_usdt.then(
            // Create a promise to callback query_greeting_callback
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(5))
                .query_greeting_callback(
                    //TODO ??
                    // AccountId::from_str(NEAR_TOKEN_ID).unwrap(),
                    // Some(U128(10u128.pow(24))),
                ),
        );
        // USDT-USDC
        let amount_in_usdt_usdc = Some(U128(10u128.pow(6)));
        let promise_usdt_usdc = dex.get_return(
            USDT_USDC_USDTe_USDC_POOL_ID,
            AccountId::from_str(USDT_TOKEN_ID).unwrap(),
            amount_in_usdt_usdc,
            AccountId::from_str(USDC_TOKEN_ID).unwrap()
        );
        let amount_out_usdt_usdc = promise_usdt_usdc.then(
            // Create a promise to callback query_greeting_callback
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(5))
                .query_greeting_callback(
                    //TODO ??
                    // AccountId::from_str(NEAR_TOKEN_ID).unwrap(),
                    // Some(U128(10u128.pow(24))),
                ),
        );
        // USDC-NEAR
        let amount_in_usdc_near = Some(U128(10u128.pow(6)));
        let promise_usdc_near = dex.get_return(
            NEAR_USDC_POOL_ID,
            AccountId::from_str(USDC_TOKEN_ID).unwrap(),
            amount_in_usdc_near,
            AccountId::from_str(NEAR_TOKEN_ID).unwrap()
        );
        let amount_out_usdc_near = promise_usdc_near.then(
            // Create a promise to callback query_greeting_callback
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(5))
                .query_greeting_callback(
                    //TODO ??
                    // AccountId::from_str(NEAR_TOKEN_ID).unwrap(),
                    // Some(U128(10u128.pow(24))),
                ),
        );
        // NEAR-USDT
        let out1 = BigDecimal::from_str(&amount_out_near_usdt.to_string().unwrap()) /
            BigDecimal::from_str(&10u128.pow(6).to_string().unwrap());
        let in1 = BigDecimal::from_str(&amount_in_near_usdt.to_string().unwrap());
        let rate1 = out1.div(in1);
        // USDT-USDC
        let out2 = BigDecimal::from_str(&amount_out_usdt_usdc.to_string().unwrap()) /
            BigDecimal::from_str(&10u128.pow(6).to_string().unwrap());
        let in2 = BigDecimal::from_str(&amount_in_usdt_usdc.to_string().unwrap());
        let rate2 = out2.div(in2);
        // USDC-NEAR
        let out3 = BigDecimal::from_str(&amount_out_usdc_near.to_string().unwrap()) /
            BigDecimal::from_str(&10u128.pow(24).to_string().unwrap());
        let in3 = BigDecimal::from_str(&amount_in_usdc_near.to_string().unwrap());
        let rate3 = out3.div(in3);

        let gain_multiplier = rate1.mul(rate2).mul(rate3);
        if gain_multiplier > BigDecimal::from(1) {
            return gain_multiplier.sub(BigDecimal::from(1))
        }
        else {
            return None
        }

    }
    fn trade(&self, amount: BigDecimal, expected_gain: BigDecimal) -> Option<BigDecimal> {
        let gain = self.get_gain_potential();
        if gain == None {
            return None
        }
        let inventory = self.inventory.get(&Token::NEAR).unwrap().into();
        let amount_in = inventory.mul(&self.trade_size.into().copy());

        let swap_actions = vec![SwapAction{
            pool_id: NEAR_USDT_POOL_ID,
            token_in: AccountId::from_str(NEAR_TOKEN_ID).unwrap(),
            amount_in: Some(U128(amount_in) * U128(10u128.pow(24))),
            token_out: AccountId::from_str(USDT_TOKEN_ID).unwrap(),
            min_amount_out: U128(0),
        }, SwapAction{
            pool_id: USDT_USDC_USDTe_USDC_POOL_ID,
            token_in: AccountId::from_str(USDT_TOKEN_ID).unwrap(),
            amount_in: None,
            token_out: AccountId::from_str(USDC_TOKEN_ID).unwrap(),
            min_amount_out: U128(0),
        }, SwapAction{
            pool_id: NEAR_USDC_POOL_ID,
            token_in: AccountId::from_str(USDC_TOKEN_ID).unwrap(),
            amount_in: None,
            token_out: AccountId::from_str(NEAR_TOKEN_ID).unwrap(),
            min_amount_out: U128(0),
        }];

    }



}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct BigDeDecimal {
    positive: bool,
    digits: Vec<u32>,
    scale: i64
}

impl Eq for BigDeDecimal {}

impl PartialOrd for BigDeDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        BigDecimal::from(self).partial_cmp(&BigDecimal::from(other))
    }
}

impl Ord for BigDeDecimal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        BigDecimal::from(self).cmp(&BigDecimal::from(other))
    }
}

impl From<BigDecimal> for BigDeDecimal {
    fn from(value: BigDecimal) -> Self {
        let (bigint, scale) = value.as_bigint_and_exponent();
        let (sign, digits) = bigint.to_u32_digits();
        let positive = sign != Sign::Minus;
        BigDeDecimal {positive, digits, scale}
    }
}

impl From<&BigDecimal> for BigDeDecimal {
    fn from(value: &BigDecimal) -> Self {
        value.clone().into()
    }
}

impl From<BigDeDecimal> for BigDecimal {
    fn from(value: BigDeDecimal) -> Self {
        let BigDeDecimal {positive, digits, scale} = value;
        let sign = if positive {Sign::Plus} else {Sign::Minus};
        let bigint = bigdecimal::num_bigint::BigInt::new(sign, digits);
        BigDecimal::new(bigint, scale)
    }
}

impl From<&BigDeDecimal> for BigDecimal {
    fn from(value: &BigDeDecimal) -> Self {
        value.clone().into()
    }
}
