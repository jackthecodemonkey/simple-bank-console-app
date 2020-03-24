use super::super::models::AccountModel::Account;
use super::super::models::TransferModel::Transfer;

pub trait BankServiceTrait {
    fn LoadData(&mut self) -> &'static str;
    fn AddAccount(&mut self, account: Account) -> &'static str;
    fn DeleteAccount(&mut self, account_no: u32) -> &'static str;
    fn Deposit(&mut self, account_no: u32, amount: i128) -> &'static str;
    fn Withdraw(&mut self, account_no: u32, amount: i128) -> &'static str;
    fn Transfer(&mut self, transfer: Transfer) -> &'static str;
}