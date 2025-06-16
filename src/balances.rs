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

    /// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: String,
		to: String,
		amount: u128,
	) -> Result<(), &'static str> {
		let caller_balance = self.get_balance(&caller);
        let to_balance = self.get_balance(&to);
        // The chained `ok_or` along with `?` follows the pattern:
        // If checked_sub returns None, we will make the function to return an Err with the message "Not enough funds."
        // that can be displayed to the user.
        // Otherwise, if checked_sub returns Some(value), we will assign new_from_balance directly to that value.
        // In this case, we are writing code which completely handles the Option type in a safe and ergonomic way.
        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Not enough funds.")?;
        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow error.")?;
        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

		Ok(())
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

    #[test]
	fn transfer_balance() {
		/* This test checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.
		*/
		let mut balances = super::Pallet::new();
		let transfer_amount = 10;

		let ini_alice_balance = balances.get_balance(&"alice".to_string());
		assert_eq!(ini_alice_balance, 0);
		
		let mut res = balances.transfer("alice".to_string(), "bob".to_string(), transfer_amount);
		assert_eq!(res, Err("Not enough funds."));

		balances.set_balance(&"alice".to_string(), 100);
		let new_alice_balance = balances.get_balance(&"alice".to_string());
		assert_eq!(new_alice_balance, 100);

		res = balances.transfer("alice".to_string(), "bob".to_string(), transfer_amount);
		assert_eq!(res, Ok(()));
		let end_alice_balance = balances.get_balance(&"alice".to_string());
		let end_bob_balance = balances.get_balance(&"bob".to_string());
		assert_eq!(end_alice_balance, 90);
		assert_eq!(end_bob_balance, 10);
	}
}
