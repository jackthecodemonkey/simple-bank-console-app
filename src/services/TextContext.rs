use std::fs::File;

use super::super::models::AccountModel::Account;
use super::super::models::TransferModel::Transfer;
use super::super::traits::BankServiceTrait::BankServiceTrait;

use std::io::prelude::*;

#[derive(Debug)]
pub struct TextContext {
    pub dbContext: File,
}

impl BankServiceTrait for TextContext {
    fn LoadData (&mut self) -> &'static str {
        let mut contents = String::new();
        let buf_content: usize = match self.dbContext.read_to_string(&mut contents) {
            Ok(content) => content,
            Err(_) => 10,
        };

        println!("{:?}", contents);

        "Not implemented yet"
    }
    fn AddAccount(&mut self, account: Account) -> &'static str {
        "Not implemented yet"
    }
    fn DeleteAccount(&mut self, account_no: u32) -> &'static str {
        "Not implemented yet"
    }
    fn Deposit(&mut self, account_no: u32, amount: i128) -> &'static str {
        "Not implemented yet"
    }
    fn Withdraw(&mut self, account_no: u32, amount: i128) -> &'static str {
        "Not implemented yet"
    }
    fn Transfer(&mut self, transfer: Transfer) -> &'static str {
        "Not implemented yet"
    }
}
