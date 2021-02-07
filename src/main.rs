mod account;
mod positive_f64;
mod transaction;
mod block;
mod blockchain;

use crate::{
	transaction::Transaction,
	blockchain::BlockChain,
	account::Account,
};

// TODO:
// capire come fare i test negli esempi
// far funzionare i test
// aggiungere il tempo di creazione di tutte le cose che lo richiedono
//     modificare di conseguenza i vari hash
// effettua transazione
// RISOLVI PROBLEMA &mut Account
// impl sub pf64

fn main() {
	let a0 = Account::new("a", "a", "a");
	let a1 = Account::new("b", "b", "b");
	let a2 = Account::new("c", "c", "c");
	let a3 = Account::new("d", "d", "d");
	let a4 = Account::new("e", "e", "e");
	let a5 = Account::new("f", "f", "f");

	let t0 = Transaction::new(a0, a1, 5.1, "a"); // THEY ARE NOT UPDATED
	let t1 = Transaction::new(a2, a3, 1.0, "c");
	let t2 = Transaction::new(a4, a5, -2.0, "e");

	let mut blockchain = BlockChain::new(2);
	blockchain.push_transaction(t0);
	blockchain.push_transaction(t1);
	blockchain.push_transaction(t2);

	println!("{:#?}", blockchain);
	//println!("{}, {}", a0, a1)
}
