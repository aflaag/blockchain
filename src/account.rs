use sha2::{Sha512, Digest};
use std::convert::TryInto;
use rand::rngs::OsRng;
use std::fmt;
use ed25519_dalek::Keypair;
use crate::positive_f64::PositiveF64;

/// A structure to handle accounts for the currency.
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
	first_name: String,
	last_name: String,
	balance: PositiveF64,
	keypair: [u8; 64],
	hash_password: [u8; 64],
}

impl Account {
	/// Generates a new `Account`.
	/// Every account has a first name, a last name, a balance and a password, which is used to validate the transactions;
	/// the password is saved using the SHA-512 hashing algorithm.
	/// Also, every account has a `Keypair` which is used to validate the signature of the transaction,
	/// using the `ed25519_dalek` crate.
	/// 
	/// # Example
	///
	/// ```
	/// // TODO: HERE
	/// let ferris = Account::new("Ferris", "Rusty", "I_Love_Ferris_123!#"); // make sure your password is safe enough!
	/// 
	/// //assert_eq!(1, 2);
	/// ```
	pub fn new(first_name: &str, last_name: &str, password: &str) -> Self {
		let mut csprng = OsRng;
		let keypair: Keypair = Keypair::generate(&mut csprng);

		let mut hasher = Sha512::new();

		hasher.update(password.as_bytes());

		let hash_password = hasher
			.finalize()[..]
			.try_into()
			.expect("Error generating the SHA-512 hash of the password.");

		Self {
			first_name: String::from(first_name),
			last_name: String::from(last_name),
			balance: PositiveF64::new(0.0).unwrap(),
			keypair: keypair.to_bytes(),
			hash_password,
		}
	}

	// TODO: HERE
	/// A method to add money to your balance; the amount can't be `0.0`, and can't be negative.
	/// 
	/// # Example
	/// ```
	/// ```
	pub fn add_money(&mut self, amount: f64) {
		if amount == 0.0 {
			eprintln!("Can't add a zero-value amount to the balance.")
		} else {
			match PositiveF64::new(amount) {
				Ok(a) => self.balance += a,
				Err(e) => eprintln!("{} Details: can't add a negative amount to the balance.", e),
			}
		}
	}


	// TODO: HERE
	/// A method to subtract money to your balance; the amount to subtract can't be `0.0`, can't be negative,
	/// and can't be more than the amount in your balance.
	/// 
	/// # Example
	/// ```
	/// ```
	#[allow(dead_code)]
	pub fn sub_money(&mut self, amount: f64) {
		if amount == 0.0 {
			eprintln!("Can't subtract a zero-value amount to the balance.")
		} else {
			match PositiveF64::new(amount) {
				Ok(a) => {
					if PositiveF64::new(self.balance.value() - a.value()).is_ok() { // if the difference is >= 0.0
						self.balance -= a
					} else {
						eprintln!("Can't subtract an amount that is more than the amount in your balance.")
					}
				},
				Err(e) => eprintln!("{} Details: can't subtract a negative amount to the balance.", e)
			}
		}
	}

	// TODO: HERE
	/// This method returns the balance of the account, since the `balance` field isn't `pub`.
	pub fn balance(&self) -> f64 {
		self.balance.value()
	}

	// TODO: HERE
	/// This method returns the keypair of the account, since the `keypair` field isn't `pub`.
	pub fn keypair(&self) -> [u8; 64] {
		self.keypair
	}

	// TODO: HERE
	/// This method returns the hash of the password of the account, since the `hash_password` field isn't `pub`.
	pub fn hash_password(&self) -> [u8; 64] {
		self.hash_password
	}

	// TODO: HERE
	/// A method to add money to your balance, without checking if the amount is non-zero and positive.
	/// 
    	/// # Safety
  	/// Adds money to the balance without checking if the amount is valid.
   	/// 
    	/// # Examples
    	/// 
    	/// ```
    	/// unsafe {
	/// 
    	/// }
    	/// ```
    	/// 
    	/// # Panics
    	/// 
    	/// The invalid amount could lead to uncertain behaviour with calculations.
    	/// 
    	/// ```should_panic
    	/// let mut john = Account::new("John", "Keats", "my_password2021!");
    	/// 
	/// unsafe {
	/// 	john.add_money_unchecked(-4.0);
	/// 
	/// 	// The above expression could make the program panic!
    	/// }
    	/// ```
	pub unsafe fn add_money_unchecked(&mut self, amount: f64) {
		self.balance += PositiveF64::new_unchecked(amount)
	}

	// TODO: HERE
	/// A method to subtract money to your balance, without checking if the amount is non-zero and positive.
	/// 
    	/// # Safety
    	/// Subtracts money to the balance without checking if the amount is valid.
    	/// 
    	/// # Examples
    	/// 
    	/// ```
    	/// unsafe {
	/// 
    	/// }
    	/// ```
    	/// 
    	/// # Panics
    	/// 
    	/// The invalid amount could lead to uncertain behaviour with calculations.
    	/// 
    	/// ```should_panic
    	/// let mut john = Account::new("John", "Keats", "my_password2021!");
	/// 
	/// unsafe {
	/// 	john.sub_money_unchecked(-5.0);
	/// 
	/// 	// The above expression could make the program panic!
    	/// }
    	/// ```
	pub unsafe fn sub_money_unchecked(&mut self, amount: f64) {
		self.balance -= PositiveF64::new_unchecked(amount)
	}
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {}: {})", self.first_name, self.last_name, self.balance)
    }
}
