use std::{fmt, error};
use std::convert::TryInto;
use sha2::{Sha512, Digest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
	pub sender: String,
	pub receiver: String,
	pub amount: u64,
	pub hash: [u8; 64]
}

impl Transaction {
	pub fn new(sender: &str, receiver: &str, amount: u64) -> Self {
		let mut transaction = Self {
			sender: String::from(sender),
			receiver: String::from(receiver),
			amount,
			hash: [0; 64],
		};

		transaction.calculate_hash();

		transaction
	}

	fn calculate_hash(&mut self) {
		let mut hasher = Sha512::new();

		let digest = format!("{}{}{}", self.sender, self.receiver, self.amount);

		hasher.update(digest.as_bytes());

		self.hash = hasher
			.finalize()[..]
			.try_into()
			.expect("Error generating the SHA-512 hash for the transaction.");
	}
	
	pub fn validate(&self, hash: [u8; 64]) -> Result<(), ValidationError> {
		if hash != self.hash {
			Err(ValidationError::Tempered)
		} else {
			Ok(())
		}
	}
}

#[derive(Debug)]
pub enum ValidationError {
	Tempered,
	InvalidSign,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tempered transaction.")
    }
}

impl error::Error for ValidationError {}
