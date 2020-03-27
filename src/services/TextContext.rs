use std::fs::File;

use super::super::models::AccountModel::Account;
use super::super::models::AccountsModel::Accounts;
use super::super::models::TransferModel::Transfer;
use super::super::traits::BankServiceTrait::BankServiceTrait;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct TextContext {
    pub dbContext: File,
    pub openOptions: File,
}

impl BankServiceTrait for TextContext {
    fn LoadData(&mut self) -> Accounts {
        let mut contents = String::new();
        self.dbContext.read_to_string(&mut contents);
        Accounts {
            accounts: contents
                .split("\r\n")
                .map(|x| -> Account {
                    let sliced: Vec<&str> = x.split(",").collect();
                    if sliced.len() != 3 {
                        panic!("data is currupted");
                    }
                    Account {
                        no: FromStr::from_str(sliced[0]).unwrap(),
                        name: String::from(sliced[1]),
                        deposit: FromStr::from_str(sliced[2]).unwrap(),
                    }
                })
                .collect(),
        }
    }
    fn AddAccount(&mut self, account: Account) -> &'static str {
        let csv_account: String = account.Stringify();
        if let Err(e) = write!(self.openOptions, "{}", csv_account.as_str()) {
            return "Failed to add a new account";
        }
        return "Account added successfully";
    }
    fn DeleteAccount(&mut self, account_no: u32) -> &'static str {
        let allAccounts: Accounts = self.LoadData();
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
