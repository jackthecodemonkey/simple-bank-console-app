use super::super::models::account_model::Account;
use super::super::models::accounts_model::Accounts;
use super::super::models::transfer_model::Transfer;

pub trait bank_service_trait {
    fn load_data(&mut self) -> Accounts;
    fn add_account(&mut self, account: Account) -> Result<Account, &str>;
    fn delete_account(&mut self, account_no: i32) -> &'static str;
    fn deposit(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str>;
    fn withdraw(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str>;
    fn transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str>;
}