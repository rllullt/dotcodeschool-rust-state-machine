use std::collections::BTreeMap;

// State and entry point of this module
// For a balance system, we really only need to keep track of one thing: how much balance each user has in our system.
pub struct Pallet {
    balances: BTreeMap<String, u128>,  // u128: largest native type. This will allow users to have ver, very large balances.
}

impl Pallet {
    /// Create a new instance of the balances module
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`
    pub fn get_balance(&mut self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
        // same as return *self...;
        // Note: get returns an Option object
        // Option: Some(value) | None
        // unwrap returns the value of Some(value) [may fail if it is a None]
        // unwrap_or returns the value of Some(value) or a provided default
    }
}

// Let’s test!
#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
        let mut balances = super::Pallet::new();

        assert_eq!(balances.get_balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.get_balance(&"alice".to_string()), 100);
        assert_eq!(balances.get_balance(&"bob".to_string()), 0);
	}
}
