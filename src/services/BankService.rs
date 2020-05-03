use super::super::models::account_model::Account;
use super::super::models::accounts_model::Accounts;
use super::super::models::transfer_model::Transfer;
use super::super::traits::BankServiceTrait::bank_service_trait;

#[derive(Debug)]
pub struct BankService<T> {
    pub db_context: T,
}

impl<T> BankService<T> where T: bank_service_trait {
    pub fn new(db_context: T) -> BankService<T> {
        BankService {
            db_context,
        }
    }
}

impl<T> bank_service_trait for BankService<T>
where
    T: bank_service_trait,
{
    fn load_data(&mut self) -> Accounts {
        self.db_context.load_data()
    }
    fn add_account(&mut self, account: Account) -> Result<Account, &str> {
        self.db_context.add_account(account)
    }
    fn delete_account(&mut self, account_no: i32) -> &'static str {
        self.db_context.delete_account(account_no)
    }
    fn deposit(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        self.db_context.deposit(account_no, amount)
    }
    fn withdraw(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        self.db_context.withdraw(account_no, amount)
    }
    fn transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str> {
        self.db_context.transfer(transfer)
    }
}
