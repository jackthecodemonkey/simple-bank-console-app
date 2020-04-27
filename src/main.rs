#[macro_use]
extern crate diesel;
extern crate dotenv;

mod dbConnections;
mod models;
mod services;
mod traits;
pub mod schema;
use std::env;
use models::Commands::Commands;
use models::FileContext::FileContext;
use models::View::View;
use models::AccountModel::Account;
use models::AccountsModel::Accounts;
use models::TransferModel::Transfer;
use services::BankService::BankService;
use services::FileDBContext::FileDBContext;
use dbConnections::db_connection::establish_connection;
use schema::accounts::dsl::*;
use schema::transactions::dsl::*;
use diesel::prelude::*;

use traits::BankServiceTrait::BankServiceTrait;

pub struct DBContext<'a> {
    context: schema::accounts::table,
    transaction_context: schema::transactions::table,
    connection: &'a PgConnection
}

impl<'a> BankServiceTrait for DBContext<'a> {
    fn LoadData(&mut self) -> Accounts {
        let results = self.context.load::<Account>(self.connection).expect("Error loading accounts");
        Accounts {
            accounts: results
        }
    }
    fn AddAccount(&mut self, account: Account) -> Result<Account, &str> {
        let account = diesel::insert_into(accounts).values(&account).get_result::<Account>(self.connection).expect("Error saving new account");
        Ok(account)
    }
    fn DeleteAccount(&mut self, account_no: i32) -> &'static str {
       "Not implemented yet"
    }
    fn Deposit(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        Err("Not implemented yet")
    }
    fn Withdraw(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        Err("Not implemented yet")
    }
    fn Transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str> {
        Err("Not implemented yet")
    }
}

fn main() {
    let connection = establish_connection();

    let mut dbContext: DBContext = DBContext {
        context: accounts,
        transaction_context: transactions,
        connection: &connection
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
