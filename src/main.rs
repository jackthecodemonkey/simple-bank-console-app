#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::default::Default;
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
use services::SQLContext::SQLContext;
use models::View::View;
use schema::accounts::dsl::*;
use schema::transactions::dsl::{no as transaction_number, transactions};
use services::BankService::BankService;
use services::FileDBContext::FileDBContext;
use std::env;

use models::TransactionModel::{Transaction as TransactionModel, NewTransaction};
use traits::BankServiceTrait::BankServiceTrait;



fn main() {
    let connection = establish_connection();

    let mut dbContext: SQLContext = SQLContext {
        context: accounts,
        transaction_context: transactions,
        connection: &connection,
    };

    dbContext.Deposit(1,23400.0);

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
