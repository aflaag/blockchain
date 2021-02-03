use crate::Transaction;

use std::convert::TryInto;
use sha2::{Sha512, Digest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
index: usize,
    prev_hash: [u8; 64],
	transactions: Vec<Transaction>,
	nonce: u128,
	// time:
	pub hash: [u8; 64],
}

impl Block {
	pub fn new(index: usize, prev_hash: [u8; 64], transactions: Vec<Transaction>) -> Self {
		let mut block = Self {
			index,
			prev_hash,
			transactions,
			nonce: 0,
			hash: [0; 64],
		};

		block.calculate_hash();

		block
	}

	fn calculate_hash(&mut self) {
		while &self.hash[0..2] != [69, 69] {
			let mut hasher = Sha512::new();

			// str(index) + str(ph) + "\"\\\"\\\"" + str(t1) + "\""  + str(t2)
			let transactions_hashes = self.transactions.iter().fold(String::new(), |acc, t| format!("{:?}{:?}", acc, t.hash));
	
			let digest = format!("{:?}{:?}{}{}", self.index, self.prev_hash, transactions_hashes, self.nonce);
	
			hasher.update(digest.as_bytes());
			
			self.hash = hasher
				.finalize()[..]
				.try_into()
				.expect("Error generating ");

			self.nonce += 1;
		}
	}
}

impl Default for Block {
	fn default() -> Self {
		Block::new(0, [0; 64], Vec::new())
	}
}
