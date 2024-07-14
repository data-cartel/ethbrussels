use std::str::FromStr;
use near_sdk::{env, json_types::U128, log, near, AccountId, Gas, Promise};

mod dex;
use dex::*;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    dex_id: AccountId,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        let dex_id = AccountId::from_str("ref-finance-101.testnet").unwrap();
        Self { dex_id }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn step(&mut self) -> Promise {
        let greeting = "placeholder".to_string(); //
        log!("Saving greeting: {greeting}");

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
