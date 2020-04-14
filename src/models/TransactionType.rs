extern crate chrono;
use chrono::prelude::*;

pub enum TransactionType {
    Deposit,
    Withdraw,
    Transfer,
}

impl TransactionType {
    pub fn get_transction_content(&self, content: String) -> String {
        let now = Local::now();
        let mut string = String::from(content);
        if string != "\r\n" {
            string.push_str(", ");
        }
        match self {
            TransactionType::Deposit => {
                string.push_str("type: deposit");
            }
            TransactionType::Withdraw => {
                string.push_str("type: withdraw");
            }
            TransactionType::Transfer => {
                string.push_str("type: transfer");
            }
        };
        string.push_str(" ,date: ");
        string.push_str(now.to_rfc3339().as_str());
        string
    }
}
