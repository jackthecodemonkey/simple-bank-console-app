use super::super::models::AccountModel::Account;
use super::super::models::AccountsModel::Accounts;
use super::super::models::TransactionType::TransactionType;
use super::super::models::TransferModel::Transfer;
use super::super::traits::BankServiceTrait::BankServiceTrait;
use super::super::traits::Transaction::Transaction;

#[derive(Debug)]
pub struct BankService<T, M> {
    pub dbContext: T,
    pub transactionContext: M,
}

impl<T, M> BankService<T, M> {
    pub fn new(dbContext: T, transactionContext: M) -> BankService<T, M> {
        BankService {
            dbContext,
            transactionContext,
        }
    }
}

impl<T, M> BankServiceTrait for BankService<T, M>
where
    T: BankServiceTrait,
    M: Transaction,
{
    fn LoadData(&mut self) -> Accounts {
        self.dbContext.LoadData()
    }
    fn AddAccount(&mut self, account: Account) -> Result<Account, &str> {
        self.dbContext.AddAccount(account)
    }
    fn DeleteAccount(&mut self, account_no: u32) -> &'static str {
        self.dbContext.DeleteAccount(account_no)
    }
    fn Deposit(&mut self, account_no: u32, amount: i128) -> Result<Accounts, &str> {
        match self.dbContext.Deposit(account_no, amount) {
            Ok(account) => {
                let stringify = &account.Stringify();
                self.transactionContext
                    .store_history(TransactionType::Deposit(String::from("Deposit")), stringify);
                Ok(account)
            }
            Err(err) => Err(err),
        }

        // self.dbContext.Deposit(account_no, amount)
    }
    fn Withdraw(&mut self, account_no: u32, amount: i128) -> Result<Accounts, &str> {
        self.dbContext.Withdraw(account_no, amount)
    }
    fn Transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str> {
        self.dbContext.Transfer(transfer)
    }
}
