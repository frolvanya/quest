// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, Gas, NearToken, Promise};
use serde_json::json;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    greeting: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {greeting}");
        self.greeting = greeting;
    }

    pub fn solve_quest(&mut self) -> Promise {
        // Proxy a call to the existing contract's method
        Promise::new("birthday-quest.near".parse().unwrap()).function_call(
            "happy_birthday".to_string(),
            json!({ "hash": "015eb7da5050320740a4271810772cd58571fb96fdb69aebc70ff2640856829d" })
                .to_string()
                .into_bytes(),
            NearToken::from_near(0),           // attached deposit
            Gas::from_gas(10_000_000_000_000), // attached gas
        )
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
