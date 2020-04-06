extern crate chrono;
use chrono::prelude::*;

pub enum TransactionType {
    Deposit(String),
    Withdraw(String),
    Transfer((String, String)),
}

impl TransactionType {
    pub fn get_transction_content(&self, content: &str) -> &str {
        println!("time: {:?}", Local::now());
        let mut transaction_content: String = match self {
            TransactionType::Deposit(ref s) => String::from("Deposit"),
            TransactionType::Withdraw(ref s) => String::from("Withdraw"),
            TransactionType::Transfer((ref from, ref to)) => String::from("Transfer"),
        };
        "Test"
    }
}
