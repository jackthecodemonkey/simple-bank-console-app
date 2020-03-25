use super::super::models::AccountModel::Account;
use super::super::models::AccountsModel::Accounts;
use super::super::models::TransferModel::Transfer;
use super::super::traits::BankServiceTrait::BankServiceTrait;

#[derive(Debug)]
pub struct SQLContext {
    pub dbContext: u32,
}

impl BankServiceTrait for SQLContext {
    fn LoadData(&mut self) -> Accounts {
        Accounts {
            accounts: Vec::new()
        }
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
