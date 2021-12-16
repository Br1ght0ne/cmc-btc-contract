use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct BTCPrice {
    pub last_prices: [f64; 5],
}

#[near_bindgen]
impl BTCPrice {
    pub fn add_price(&mut self, price: f64) {
        self.last_prices.rotate_right(1);
        self.last_prices[0] = price;
    }

    pub fn average_price(&self) -> f64 {
        let prices = self
            .last_prices
            .iter()
            .copied()
            .filter(|price| *price > 0.0)
            .collect::<Vec<_>>();
        if !prices.is_empty() {
            prices.iter().sum::<f64>() / prices.len() as f64
        } else {
            0.0
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn test_add_price() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice);
        testing_env!(context.build());

        let mut contract = BTCPrice::default();
        assert_approx_eq!(contract.last_prices[0], 0.);
        contract.add_price(1000.0);
        assert_approx_eq!(contract.last_prices[0], 1000.0);
    }

    #[test]
    fn test_average_price() {
        // Get Alice as an account ID
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice);
        testing_env!(context.build());

        let mut contract = BTCPrice::default();
        assert_approx_eq!(contract.average_price(), 0.);

        contract.add_price(1000.0);
        assert_approx_eq!(contract.average_price(), 1000.0);

        contract.add_price(2000.0);
        assert_approx_eq!(contract.average_price(), 1500.0);

        for i in 3..=5 {
            contract.add_price(1000.0 * i as f64);
        }
        assert_approx_eq!(contract.average_price(), 3000.0);

        contract.add_price(6000.0);
        assert_approx_eq!(contract.average_price(), 4000.0);
    }
}
