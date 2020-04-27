use super::super::models::AccountModel::Account;
use super::super::models::AccountsModel::Accounts;
use super::super::models::TransactionType::TransactionType;
use super::super::models::TransferModel::Transfer;
use super::super::traits::BankServiceTrait::BankServiceTrait;
use super::super::traits::Transaction::Transaction;
use super::super::services::FileDBContext::FileDBContext;

#[derive(Debug)]
pub struct BankService<T> {
    pub dbContext: T,
}

impl<T> BankService<T> {
    pub fn new(dbContext: T) -> BankService<T> {
        BankService {
            dbContext,
        }
    }
}

impl<T> BankServiceTrait for BankService<T>
where
    T: BankServiceTrait,
{
    fn LoadData(&mut self) -> Accounts {
        self.dbContext.LoadData()
    }
    fn AddAccount(&mut self, account: Account) -> Result<Account, &str> {
        self.dbContext.AddAccount(account)
    }
    fn DeleteAccount(&mut self, account_no: i32) -> &'static str {
        self.dbContext.DeleteAccount(account_no)
    }
    fn Deposit(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        self.dbContext.Deposit(account_no, amount)
    }
    fn Withdraw(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        self.dbContext.Withdraw(account_no, amount)
    }
    fn Transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str> {
        self.dbContext.Transfer(transfer)
    }
}
