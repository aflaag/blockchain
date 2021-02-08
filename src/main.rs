mod account;
mod positive_f64;
mod transaction;
mod block;
mod blockchain;

use crate::{
    blockchain::BlockChain,
    account::Account,
};

// TODO:
// capire come fare i test negli esempi
// far funzionare i test

fn main() {
    let mut a0 = Account::new("a", "a", "a");
    let mut a1 = Account::new("b", "b", "b");
    let mut a2 = Account::new("c", "c", "c");
    let mut a3 = Account::new("d", "d", "d");
    let mut a4 = Account::new("e", "e", "e");
    //let mut a5 = Account::new("f", "f", "f");
    
    a0.add_money(100.0);
    a2.add_money(100.0);
    a4.add_money(100.0);

    let mut blockchain = BlockChain::new(2);
    blockchain.push_transaction(&mut a0, &mut a1, 2.0, "a");
    blockchain.push_transaction(&mut a2, &mut a3, 1.0, "c");

    println!("{} {} {} {} {}", a0, a1, a2, a3, a4);
}
