use super::super::models::AccountModel::Account;
use super::super::models::AccountsModel::Accounts;
use super::super::models::TransferModel::Transfer;
use super::super::traits::BankServiceTrait::BankServiceTrait;

use super::super::schema::accounts::dsl::*;
use super::super::schema::transactions::dsl::transactions;

use diesel::prelude::*;
use super::super::models::TransactionType::TransactionType;
use super::super::models::TransactionModel::{Transaction as TransactionModel, NewTransaction};

pub struct SQLContext<'a> {
    pub context: super::super::schema::accounts::table,
    pub transaction_context: super::super::schema::transactions::table,
    pub connection: &'a PgConnection,
}

impl<'a> SQLContext<'a> {
    fn GetAccount(&self, account_no: i32) -> Result<Account, &str> {
        let acc = accounts
            .find(account_no)
            .get_result::<Account>(self.connection)
            .expect("Unable to get account");
        Ok(acc)
    }

    fn write_transaction(
        &self,
        account_no: i32,
        transaction_type: TransactionType,
        amount: f64,
        current_balance: f64,
    ) -> Result<(), &str> {
        let transaction = NewTransaction {
            no: account_no,
            transaction_type: match transaction_type {
                TransactionType::Deposit => "Deposit".to_string(),
                TransactionType::Withdraw => "Withdraw".to_string(),
                TransactionType::Transfer => "Transfer".to_string(),
            },
            transaction_amount: amount,
            current_balance,
        };
        diesel::insert_into(transactions)
            .values(transaction)
            .get_result::<TransactionModel>(self.connection)
            .expect("Error saving transaction");
        Ok(())
    }
}

impl<'a> BankServiceTrait for SQLContext<'a> {
    fn LoadData(&mut self) -> Accounts {
        let results = self
            .context
            .load::<Account>(self.connection)
            .expect("Error loading accounts");
        Accounts { accounts: results }
    }
    fn AddAccount(&mut self, account: Account) -> Result<Account, &str> {
        let account = diesel::insert_into(accounts)
            .values(&account)
            .get_result::<Account>(self.connection)
            .expect("Error saving new account");
        Ok(account)
    }
    fn DeleteAccount(&mut self, account_no: i32) -> &'static str {
        diesel::delete(accounts.filter(no.eq(&account_no)))
            .execute(self.connection)
            .expect("Error deleting account");
        "Successfullt deleted"
    }
    fn Deposit(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        if let Ok(ref mut account) = self.GetAccount(account_no) {
            let _ = account.Deposit(amount);
            let _ = diesel::update(accounts.find(account_no))
                .set(deposit.eq(account.deposit))
                .get_result::<Account>(self.connection)
                .expect("Unable to deposit");
            let accs = self.LoadData();
            let _ =self.write_transaction(account_no,TransactionType::Deposit,amount,account.deposit);
            return Ok(accs);
        }
        Err("Unable to deposit")
    }
    fn Withdraw(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        if let Ok(ref mut account) = self.GetAccount(account_no) {
            let _ = account.Withdraw(amount);
            let _ = diesel::update(accounts.find(account_no))
                .set(deposit.eq(account.deposit))
                .get_result::<Account>(self.connection)
                .expect("Unable to deposit");
            let accs = self.LoadData();
            let _ = self.write_transaction(account_no,TransactionType::Withdraw,amount,account.deposit);
            return Ok(accs);
        }
        Err("Unable to withdraw")
    }
    fn Transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str> {
        let Transfer { from, to, amount } = transfer;
        let mut from_account = self.GetAccount(from).unwrap();
        let _ = self.GetAccount(to).unwrap();
        if from_account.CanWithdraw(amount) {
            self.Withdraw(from, amount).unwrap();
            self.Deposit(to, amount).unwrap();
            let accs = self.LoadData();
            return Ok(accs);
        }
        Err("Unable to transfer")
    }
}