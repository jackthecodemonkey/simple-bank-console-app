use super::super::models::AccountModel::Account;
use super::super::models::AccountsModel::Accounts;
use super::super::models::TransferModel::Transfer;

pub trait BankServiceTrait {
    fn LoadData(&mut self) -> Accounts;
    fn AddAccount(&mut self, account: Account) -> Result<Account, &str>;
    fn DeleteAccount(&mut self, account_no: u32) -> &'static str;
    fn Deposit(&mut self, account_no: u32, amount: i128) -> Result<Accounts, &str>;
    fn Withdraw(&mut self, account_no: u32, amount: i128) -> Result<Accounts, &str>;
    fn Transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str>;
}