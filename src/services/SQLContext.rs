use super::super::models::account_model::Account;
use super::super::models::accounts_model::Accounts;
use super::super::models::transfer_model::Transfer;
use super::super::traits::BankServiceTrait::bank_service_trait;

use super::super::schema::accounts::dsl::*;
use super::super::schema::transactions::dsl::transactions;

use diesel::prelude::*;
use super::super::models::Transaction_type::Transaction_type;
use super::super::models::transaction_model::{Transaction as transaction_model, NewTransaction};

pub struct SQLContext {
    pub context: super::super::schema::accounts::table,
    pub transaction_context: super::super::schema::transactions::table,
    pub connection: PgConnection,
}

impl SQLContext {
    fn get_account(&self, account_no: i32) -> Result<Account, &str> {
        let acc = accounts
            .find(account_no)
            .get_result::<Account>(&self.connection)
            .expect("Unable to get account");
        Ok(acc)
    }

    fn write_transaction(
        &self,
        account_no: i32,
        Transaction_type: Transaction_type,
        amount: f64,
        current_balance: f64,
    ) -> Result<(), &str> {
        let transaction = NewTransaction {
            no: account_no,
            Transaction_type: match Transaction_type {
                Transaction_type::Deposit => "deposit".to_string(),
                Transaction_type::Withdraw => "withdraw".to_string(),
                Transaction_type::Transfer => "Transfer".to_string(),
            },
            transaction_amount: amount,
            current_balance,
        };
        diesel::insert_into(transactions)
            .values(transaction)
            .get_result::<transaction_model>(&self.connection)
            .expect("Error saving transaction");
        Ok(())
    }
}

impl bank_service_trait for SQLContext {
    fn load_data(&mut self) -> Accounts {
        let results = self
            .context
            .load::<Account>(&self.connection)
            .expect("Error loading accounts");
        Accounts { accounts: results }
    }
    fn add_account(&mut self, account: Account) -> Result<Account, &str> {
        let account = diesel::insert_into(accounts)
            .values(&account)
            .get_result::<Account>(&self.connection)
            .expect("Error saving new account");
        Ok(account)
    }
    fn delete_account(&mut self, account_no: i32) -> &'static str {
        diesel::delete(accounts.filter(no.eq(&account_no)))
            .execute(&self.connection)
            .expect("Error deleting account");
        "Successfullt deleted"
    }
    fn deposit(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        if let Ok(ref mut account) = self.get_account(account_no) {
            let _ = account.deposit(amount);
            let _ = diesel::update(accounts.find(account_no))
                .set(deposit.eq(account.deposit))
                .get_result::<Account>(&self.connection)
                .expect("Unable to deposit");
            let accs = self.load_data();
            let _ =self.write_transaction(account_no,Transaction_type::Deposit,amount,account.deposit);
            return Ok(accs);
        }
        Err("Unable to deposit")
    }
    fn withdraw(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        if let Ok(ref mut account) = self.get_account(account_no) {
            let _ = account.withdraw(amount);
            let _ = diesel::update(accounts.find(account_no))
                .set(deposit.eq(account.deposit))
                .get_result::<Account>(&self.connection)
                .expect("Unable to deposit");
            let accs = self.load_data();
            let _ = self.write_transaction(account_no,Transaction_type::Withdraw,amount,account.deposit);
            return Ok(accs);
        }
        Err("Unable to withdraw")
    }
    fn transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str> {
        let Transfer { from, to, amount } = transfer;
        let mut from_account = self.get_account(from).unwrap();
        let _ = self.get_account(to).unwrap();
        if from_account.can_withdraw(amount) {
            self.withdraw(from, amount).unwrap();
            self.deposit(to, amount).unwrap();
            let accs = self.load_data();
            return Ok(accs);
        }
        Err("Unable to transfer")
    }
}