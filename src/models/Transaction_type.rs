extern crate chrono;
use chrono::prelude::*;

pub enum Transaction_type {
    Deposit,
    Withdraw,
    Transfer,
}

impl Transaction_type {
    pub fn get_transction_content(&self, content: String) -> String {
        let now = Local::now();
        let mut string = String::from(content);
        if string != "\n" {
            string.push_str(", ");
        }
        match self {
            Transaction_type::Deposit => {
                string.push_str("type: deposit");
            }
            Transaction_type::Withdraw => {
                string.push_str("type: withdraw");
            }
            Transaction_type::Transfer => {
                string.push_str("type: transfer");
            }
        };
        string.push_str(" ,date: ");
        string.push_str(now.to_rfc3339().as_str());
        string
    }
}
