mod block;
mod transaction;
mod blockchain;

use crate::{
	transaction::Transaction,
	block::Block,
	blockchain::BlockChain,
};

fn main() {
	let t0 = Transaction::new("a", "b", 0);
	let t1 = Transaction::new("c", "d", 1);
	let t2 = Transaction::new("e", "f", 2);
	let t3 = Transaction::new("g", "h", 3);
	let t4 = Transaction::new("i", "j", 4);
	let t5 = Transaction::new("k", "l", 5);
	let t6 = Transaction::new("m", "n", 6);
	let t7 = Transaction::new("o", "p", 7);
	let t8 = Transaction::new("q", "r", 8);
	
	let mut blockchain = BlockChain::new(2);
	blockchain.push_transaction(t0);
	blockchain.push_transaction(t1);
	blockchain.push_transaction(t2);
	blockchain.push_transaction(t3);
	blockchain.push_transaction(t4);
	blockchain.push_transaction(t5);
	blockchain.push_transaction(t6);
	blockchain.push_transaction(t7);
	blockchain.push_transaction(t8);

	println!("{:#?}", blockchain);
}
