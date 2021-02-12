use crate::{
    account::Account,
    transaction::{Transaction, ValidationError},
    block::Block,
};

/// A struct to handle the blockchain of the currency.
/// 
/// The treansaction contains:
/// - the index of the last block put in the chain
/// - the chain of `Block`s
/// - the pending transactions, already validated, waiting to be put in a new block
/// - the number of transactions per block
/// 
/// When the blockchain is created, it comes with the genesis block already put in the chain,
/// and the genesis is derived from the `Default` implementation of the `Block`.
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
    /// # Example
    /// ```
    /// # use blockchain::blockchain::BlockChain;
    /// let blockchain = BlockChain::new(5); // here you can choose the number of transactions per block
    /// 
    /// assert_eq!(blockchain.chain().len(), 1); // the blockchain starts with the genesis block
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

    /// This method creates a transaction with the arguments, and then this transaction is checked:
    /// if it's a valid transaction, it goes into the `Vec<Transaction>` pending transactions vector,
    /// and the amount is transferred from the sender's `Account` into the receiver's `Account`;
    /// if the transaction isn't valid, details are provided.
    /// 
    /// When the number of pending transactions is equal to the number of `transactions_per_block`,
    /// set while creating the blockchain, a new `Block` is generated.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::blockchain::BlockChain;
    /// # use blockchain::account::Account;
    /// let mut alex = Account::new("Alex", "White", "1992#?I_like_Rust92");
    /// let mut bob = Account::new("Bob", "Reds", "sUpEr_SeCuRe_PaSsWoRd#+!789");
    /// alex.add_money(100.0); // alex must have enough money to perform the transaction!
    /// 
    /// let mut blockchain = BlockChain::new(1); // the number of transactions per block is set to 1
    /// blockchain.push_transaction(&mut alex, &mut bob, 50.0, "1992#?I_like_Rust92"); // the chain is going to have two blocks, the first one being the genesis block
    /// 
    /// assert_eq!(blockchain.index, 1); // the genesis block has index #0
    /// ```
    pub fn push_transaction(&mut self, sender: &mut Account, receiver: &mut Account, amount: f64, sender_password: &str) {
        let transaction = Transaction::new(sender.clone(), receiver.clone(), amount, sender_password);

        println!("Validating transaction...");

        match transaction.validate(transaction.hash()) {
            Ok(_) => {
                self.transactions.push(transaction);

                // the amount is checked in the validation of the transaction
                unsafe {
                    sender.sub_money_unchecked(amount);
                    receiver.add_money_unchecked(amount);
                }

                println!("validated!");
            },
            Err(e) => match e {
                ValidationError::Tempered => eprintln!("{} Details: transaction from {} to {}, for an amount of {}, resulted to be tempered.",
                    e,
                    transaction.sender,
                    transaction.receiver,
                    transaction.amount(),
                ),
                ValidationError::WrongPassword => eprintln!("{} Details: the sender's password is not correct.", e),
                ValidationError::InvalidSignature => eprintln!("{} Details: transaction from {} to {}, for an amount of {}, wasn't validated because of invalid signature.",
                    e,
                    transaction.sender,
                    transaction.receiver,
                    transaction.amount(),
                ),
                ValidationError::InvalidAmount => eprintln!("{} Details: transaction from {} to {}, for an amount of {}, wasn't validated because of an invalid amount.",
                    e,
                    transaction.sender,
                    transaction.receiver,
                    transaction.amount(),
                ),
            },
        };

        if self.transactions.len() == self.transactions_per_block {
            self.index += 1;

            println!("Validating block...");

            let new_block = Block::new(
                self.index,
                self.chain.last().unwrap().hash(),
                self.transactions.clone()
            );

            self.chain.push(new_block);

            self.transactions.clear();

            println!("validated!");
        }
    }
    
    /// This method returns the `chain` of the blockchain, since this field isn't `pub`.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::blockchain::BlockChain;
    /// let blockchain = BlockChain::new(8);
    /// 
    /// assert_eq!(blockchain.chain().len(), 1); // the blockchain starts with the genesis block
    /// ```
    pub fn chain(&self) -> Vec<Block> {
        self.chain.clone()
    }
}
