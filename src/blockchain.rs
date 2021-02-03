use crate::{
	transaction::Transaction,
	block::Block,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockChain {
	index: usize,
	chain: Vec<Block>,
	transactions: Vec<Transaction>,
	transactions_per_block: usize,
}

impl BlockChain {
	pub fn new(transactions_per_block: usize) -> Self {
		let genesis_block = Block::default();

		Self {
			index: 0,
			chain: vec![genesis_block],
			transactions: Vec::new(),
			transactions_per_block,
		}
	}

	pub fn push_transaction(&mut self, transaction: Transaction) {
		println!("Validating transaction...");

		match transaction.validate(transaction.hash) {
			Ok(_) => {
				self.transactions.push(transaction);
				println!("validated!");
			},
			Err(e) => eprintln!("{} Details: transaction from {} to {}, for {}, resulted to be tempered.",
				e,
				transaction.sender,
				transaction.receiver,
				transaction.amount,
			)
		};

		if self.transactions.len() >= self.transactions_per_block {
			self.index += 1;

			println!("Validating block...");
		
			let new_block = Block::new(
				self.index,
				self.chain.last().unwrap().hash,
				self.transactions.clone()
			);

			self.chain.push(new_block);

			self.transactions.clear();

			println!("validated!");
		}
	}
}
