use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::U128;
use near_sdk::{AccountId, ext_contract};

#[allow(non_upper_case_globals, dead_code)]
pub const USDT_USDC_USDTe_USDC_POOL_ID: u64 = 4179;
pub const NEAR_USDT_POOL_ID: u64 = 3879;
#[allow(dead_code)]
pub const NEAR_USDC_POOL_ID: u64 = 4512;

pub const NEAR_TOKEN_ID: &str = "near";
pub const USDT_TOKEN_ID: &str = "usdt.tether-token.near";
#[allow(dead_code)]
pub const USDC_TOKEN_ID: &str = "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1";

#[ext_contract]
#[allow(dead_code)]
pub trait RefFi {
    /// Execute set of swap actions between pools.
    /// If referrer provided, pays referral_fee to it.
    /// If no attached deposit, outgoing tokens used in swaps must be whitelisted.
    fn swap(&mut self, actions: Vec<SwapAction>, referral_id: Option<AccountId>) -> U128;

    /// Given specific pool, returns amount of token_out recevied swapping amount_in of token_in.
    fn get_return(
        &self,
        pool_id: u64,
        token_in: AccountId,
        amount_in: U128,
        token_out: AccountId,
    ) -> U128;
}

/// Single swap action.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SwapAction {
    /// Pool which should be used for swapping.
    pub pool_id: u64,
    /// Token to swap from.
    pub token_in: AccountId,
    /// Amount to exchange.
    /// If amount_in is None, it will take amount_out from previous step.
    /// Will fail if amount_in is None on the first step.
    pub amount_in: Option<U128>,
    /// Token to swap into.
    pub token_out: AccountId,
    /// Required minimum amount of token_out.
    pub min_amount_out: U128,
}

/// Single action. Allows to execute sequence of various actions initiated by an account.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum Action {
    Swap(SwapAction),
}

/// Result from action execution.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ActionResult {
    /// No result.
    None,
    /// Amount of token was received.
    /// [AUDIT_02]
    Amount(U128),
}
