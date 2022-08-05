use sha2::{Sha512, Digest};
use std::convert::TryInto;
use rand::rngs::OsRng;
use std::fmt;
use ed25519_dalek::Keypair;
use crate::positive_f64::PositiveF64;

/// A structure to handle accounts for the currency.
/// 
/// Every account has a first name, a last name, a balance (set to 0.0) and a password,
/// which is used to validate the transactions; the password is saved using the SHA-512 hashing algorithm.
/// Also, every account has a `Keypair` which is used to validate the signature of the transaction,
/// using the `ed25519_dalek` crate.
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    first_name: String,
    last_name: String,
    balance: PositiveF64,
    keypair: [u8; 64],
    hash_password: [u8; 64],
}

impl Account {
    /// Generates a new `Account`.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::account::Account;
    /// let ferris = Account::new("Ferris", "Rusty", "I_Love_Ferris_123#@_!$%&/"); // make sure your password is safe enough!
    /// 
    /// assert_eq!(ferris.balance(), 0.0); // your balance is 0.0 when the account is created
    /// ```
    pub fn new(first_name: &str, last_name: &str, password: &str) -> Self {
        let mut csprng = OsRng;
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let mut hasher = Sha512::new();

        hasher.update(password.as_bytes());

        let hash_password = hasher
            .finalize()[..]
            .try_into()
            .expect("Error generating the SHA-512 hash of the password.");

        Self {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            balance: PositiveF64::new(0.0).unwrap(),
            keypair: keypair.to_bytes(),
            hash_password,
        }
    }

    /// A method to add money to your balance; the amount can't be `0.0`, and can't be negative.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::account::Account;
    /// let mut allen = Account::new("Allen", "Johnson", "AllenJ500321#");
    /// allen.add_money(100.0);
    /// 
    /// assert_eq!(allen.balance(), 100.0);
    /// ```
    #[allow(dead_code)]
    pub fn add_money(&mut self, amount: f64) {
        if amount == 0.0 {
            eprintln!("Can't add a zero-value amount to the balance.")
        } else {
            match PositiveF64::new(amount) {
                Ok(a) => self.balance += a,
                Err(e) => eprintln!("{} Details: can't add a negative amount to the balance.", e),
            }
        }
    }


    /// A method to subtract money to your balance; the amount to subtract can't be `0.0`, can't be negative,
    /// and can't be more than the amount in your balance.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::account::Account;
    /// let mut branda = Account::new("Branda", "Pickle", "brandA;picklE;+1992");
    /// branda.add_money(50.0); // you must have more than 0.0 in your balance
    /// 
    /// branda.sub_money(20.0);
    /// 
    /// assert_eq!(branda.balance(), 30.0); // 50.0 - 30.0 = 20.0
    /// ```
    #[allow(dead_code)]
    pub fn sub_money(&mut self, amount: f64) {
        if amount == 0.0 {
            eprintln!("Can't subtract a zero-value amount to the balance.")
        } else {
            match PositiveF64::new(amount) {
                Ok(a) => {
                    if PositiveF64::new(self.balance.value() - a.value()).is_ok() { // if the difference is >= 0.0
                        self.balance -= a
                    } else {
                        eprintln!("Can't subtract an amount that is more than the amount in your balance.")
                    }
                },
                Err(e) => eprintln!("{} Details: can't subtract a negative amount to the balance.", e)
            }
        }
    }

    /// This method returns the balance of the account, since the `balance` field isn't `pub`.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::account::Account;
    /// let mut walter = Account::new("Walter", "Clifton", "SuperWalter2000?");
    /// 
    /// assert_eq!(walter.balance(), 0.0); // your balance is 0.0 when the account is created
    /// 
    /// walter.add_money(50.0);
    /// 
    /// assert_eq!(walter.balance(), 50.0);
    /// ```
    pub fn balance(&self) -> f64 {
        self.balance.value()
    }

    /// This method returns the keypair of the account, since the `keypair` field isn't `pub`.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::account::Account;
    /// let cecilia = Account::new("Cecilia", "Lacey", "Cecilia_Is_Be@utiful49");
    /// 
    /// let cecilia_keypair = cecilia.keypair(); // this variable now contains cecilia's keypair
    /// ```
    pub fn keypair(&self) -> [u8; 64] {
        self.keypair
    }

    /// This method returns the hash of the password of the account, since the `hash_password` field isn't `pub`.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::account::Account;
    /// # use hex_literal::hex;
    /// let denzel = Account::new("Denzel", "Pratt", "My_Secret_Password@@@__789");
    /// 
    /// assert_eq!(denzel.hash_password(), hex!("de4b5227910fb4c8fa8a7702dc25807a4c6d50090615b0cd5a52446438b461071d3be479d2710ae65d48cb9fc30a3a7775f5b97a6d5b4692d17c73ab6dfd461f"));
    /// ```
    pub fn hash_password(&self) -> [u8; 64] {
        self.hash_password
    }

    /// Adds money to an account without checking the input.
    /// 
    /// # Safety
    /// A method to add money to your balance, without checking if the amount is non-zero and positive.
    /// 
    /// # Examples
    /// ```
    /// # use blockchain::account::Account;
    /// unsafe {
    ///     let mut mary = Account::new("Mary", "Shelley", "marymaryMoo123#");
    /// 
    ///     mary.add_money_unchecked(10.0);
    /// 
    ///     assert_eq!(mary.balance(), 10.0);
    /// }
    /// ```
    /// 
    /// # Panics
    /// The invalid amount could lead to uncertain behaviour with calculations.
    /// 
    /// ```
    /// # use blockchain::account::Account;
    /// unsafe {
    ///     let mut john = Account::new("John", "Keats", "my_password2021!");
    /// 
    ///     john.add_money_unchecked(-4.0);
    /// 
    ///     // The above expression could make the program panic!
    /// }
    /// ```
    pub unsafe fn add_money_unchecked(&mut self, amount: f64) {
        self.balance += PositiveF64::new_unchecked(amount)
    }

    /// Subtracts money from an account without checking the input.
    /// 
    /// # Safety
    /// A method to subtract money to your balance, without checking if the amount is non-zero and positive.
    /// 
    /// # Examples
    /// ```
    /// # use blockchain::account::Account;
    /// unsafe {
    ///     let mut mary = Account::new("Mary", "Shelley", "marymaryMoo123#");
    ///     mary.add_money(10.0); // you must have more than 0.0 in your balance
    /// 
    ///     mary.sub_money_unchecked(8.0);
    /// 
    ///     assert_eq!(mary.balance(), 2.0); // 10.0 - 8.0 = 2.0
    /// }
    /// ```
    /// 
    /// # Panics
    /// The invalid amount could lead to uncertain behaviour with calculations.
    /// 
    /// ```
    /// # use blockchain::account::Account;
    /// unsafe {
    ///     let mut john = Account::new("John", "Keats", "my_password2021!");
    ///     
    ///     john.sub_money_unchecked(-7.0);
    /// 
    ///     // The above expression could make the program panic!
    /// }
    /// ```
    pub unsafe fn sub_money_unchecked(&mut self, amount: f64) {
        self.balance -= PositiveF64::new_unchecked(amount)
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {}: {})", self.first_name, self.last_name, self.balance)
    }
}
