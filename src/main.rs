#[macro_use]
extern crate diesel;
extern crate dotenv;

mod dbConnections;
mod models;
pub mod schema;
mod services;
mod traits;
use dbConnections::db_connection::establish_connection;
use diesel::prelude::*;
use models::AccountModel::Account;
use models::AccountsModel::Accounts;
use models::Commands::Commands;
use models::FileContext::FileContext;
use models::TransactionType::TransactionType;
use models::TransferModel::Transfer;
use models::View::View;
use schema::accounts::dsl::*;
use schema::transactions::dsl::{no as transaction_number, transactions};
use services::BankService::BankService;
use services::FileDBContext::FileDBContext;
use std::env;

use traits::BankServiceTrait::BankServiceTrait;
use models::TransactionModel::TransactionModel;

pub struct DBContext<'a> {
    context: schema::accounts::table,
    transaction_context: schema::transactions::table,
    connection: &'a PgConnection,
}

impl<'a> DBContext<'a> {
    fn GetAccount(&self, account_no: i32) -> Result<Account, &str> {
        let acc = accounts
            .find(account_no)
            .get_result::<Account>(self.connection)
            .expect("Unable to get account");
        Ok(acc)
    }

    // fn wrtie_transaction(
    //     &self,
    //     transaction_type: TransactionType,
    //     amount: f64,
    //     current_balance: f64,
    // ) -> Result<(), &str> {
    //     let transaction: TransactionModel = TransactionModel {

    //     }
    //     diesel::insert_into(transactions)
    //         values()
    // }
}

impl<'a> BankServiceTrait for DBContext<'a> {
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

fn main() {
    let connection = establish_connection();

    let mut dbContext: DBContext = DBContext {
        context: accounts,
        transaction_context: transactions,
        connection: &connection,
    };

    // let results = accounts.load::<Account>(&connection)
    // .expect("Error loading accounts");

    // println!("{:?}", results);

    // let arguments: Commands = Commands::new(env::args().skip(1).collect());
    // let db_context_type = arguments.arguments[0].as_str();
    // let bank_service = match db_context_type {
    //     "use-file" => BankService::new(FileDBContext {
    //         context: FileContext::new("./src/dataSource/data.txt"),
    //         transaction_context: FileContext::new("./src/dataSource/transaction.txt"),
    //     }),
    //     "use-db" => BankService::new(FileDBContext {
    //         context: FileContext::new("./src/dataSource/data.txt"),
    //         transaction_context: FileContext::new("./src/dataSource/transaction.txt"),
    //     }),
    //     _ => panic!("Not supported db type"),
    // };

    // let mut view = View {
    //     service: bank_service,
    // };

    // view.Display();
}
