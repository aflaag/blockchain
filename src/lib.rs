pub mod account;
pub mod positive_f64;
pub mod transaction;
pub mod block;
pub mod blockchain;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
