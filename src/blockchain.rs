use crate::{
	transaction::{Transaction, ValidationError},
	block::Block,
};

/// A struct to handle the blockchain of the currency.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockChain {
	pub index: usize,
	chain: Vec<Block>,
	transactions: Vec<Transaction>,
	transactions_per_block: usize,
}

impl BlockChain {
	/// Generates a new `BlockChain`.
	/// 
	/// The treansaction contains:
	/// - the index of the last block put in the chain
	/// - the chain of `Block`s
	/// - the pending transactions, already validated, waiting to be put in a new block
	/// - the number of transactions per block
	/// 
	/// When the blockchain is created, it comes with the genesis block already put in the chain,
	/// and the genesis is derived from the `Default` implementation of the `Block`.
	/// 
	/// # Example
	/// ```
	/// let blockchain = BlockChain::new(5); // here you can choose the number of transactions per block
	/// ```
	pub fn new(transactions_per_block: usize) -> Self {
		let genesis_block = Block::default();

		Self {
			index: 0,
			chain: vec![genesis_block],
			transactions: Vec::new(),
			transactions_per_block,
		}
	}
	
	// TODO: ancora non so come si fa sempre la solita cosa
	/// When this method is called, the transaction is checked, and if it's a valid transaction,
	/// it goes into the `Vec<Transaction>` pending transactions vector.
	/// If the transaction isn't valid, details are provided.
	/// 
	/// When the number of pending transactions is equal to the number of `transactions_per_block`,
	/// set while creating the blockchain, a new `Block` is generated.
	/// 
	/// # Example
	/// ```
	/// // these accounts are taken from the `Transaction` example
	/// let alex = Account::new("Alex", "White", "1992#?I_like_Rust92");
	/// let bob = Account::new("Bob", "Reds", "sUpEr_SeCuRe_PaSsWoRd#+!789");
	/// 
	/// let transaction = Transaction::new(alex, bob, 50, "1992#?I_like_Rust92");
	/// 
	/// let blockchain = BlockChain::new(1);
	/// blockchain.push_transaction(transaction) // the blockchain is going to have two blocks, the genesis block and the block with this single transaction, since the number of transactions per block is set to 1
	/// 
	/// assert_eq(blockchain.index, 1) // the genesis block has index #0
	/// ```
	pub fn push_transaction(&mut self, transaction: Transaction) {
		println!("Validating transaction...");

		match transaction.validate(transaction.hash) {
			Ok(_) => {
				self.transactions.push(transaction);
				println!("validated!");
			},
			Err(e) => match e {
				ValidationError::Tempered => eprintln!("{} Details: transaction from {} to {}, for an amount of {}, resulted to be tempered.",
					e,
					transaction.sender,
					transaction.receiver,
					transaction.amount,
				),
				ValidationError::WrongPassword => eprintln!("{} Details: the sender's password is not correct.", e),
				ValidationError::InvalidSign => eprintln!("{} Details: transaction from {} to {}, for an amount of {}, wasn't validated because of invalid signature.",
					e,
					transaction.sender,
					transaction.receiver,
					transaction.amount,
				),
				ValidationError::InvalidAmount => eprintln!("{} Details: transaction from {} to {}, for an amount of {}, wasn't validated because of an invalid amount.",
					e,
					transaction.sender,
					transaction.receiver,
					transaction.amount,
				),
			},
		};

		if self.transactions.len() == self.transactions_per_block {
			self.index += 1;

			println!("Validating block...");
		
			self.transactions.iter_mut().for_each(|t| {
				t.sender.sub_money(t.amount);
				t.receiver.add_money(t.amount);
			});

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
