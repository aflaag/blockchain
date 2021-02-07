use crate::transaction::Transaction;
use std::convert::TryInto;
use sha2::{Sha512, Digest};
//use hex_literal::hex;

/// A structure to handle blocks for the blockchain of the currency.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
	index: usize,
	prev_hash: [u8; 64],
	transactions: Vec<Transaction>,
	nonce: u128,
	// time:
	pub hash: [u8; 64],
}

impl Block {
	/// Generates a new `Block`.
	/// Every block of the chain contains:
	/// - the index (the #0 block is the genesis block)
	/// - the SHA-512 hash of the previous block
	/// - the transactions of the block
	/// (the number of transactions per block is set while generating the blockchain)
	/// - the nonce, which is used for the proof of work
	/// - the hash of the block generated
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

	/// This method is called when a new block is generated,
	/// and it is used to calculate the SHA-512 hash of the new block.
	/// 
	/// The hash is calculated by using:
	/// - the index of the block
	/// - the previous hash
	/// - the `Transaction`s hashes
	/// - the nonce used for the proof of work
	/// 
	/// The proof of work is checked in the condition of the while loop.
	fn calculate_hash(&mut self) {
		while self.hash[0..2] != [69, 69] {
			let mut hasher = Sha512::new();

			let transactions_hashes = self.transactions.iter().fold(String::new(), |acc, t| format!("{:?}{:?}", acc, t.hash));
	
			let digest = format!("{}{:?}{}{}", self.index, self.prev_hash, transactions_hashes, self.nonce);
	
			hasher.update(digest.as_bytes());
			
			self.hash = hasher
				.finalize()[..]
				.try_into()
				.expect("Error generating the SHA-512 hash of the block.");

			self.nonce += 1;
		}
	}
}

impl Default for Block {
	fn default() -> Self {
		Block::new(0, [0; 64], Vec::new())
	}
}
