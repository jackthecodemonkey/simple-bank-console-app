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
use services::BankService::BankService;
use services::FileDBContext::FileDBContext;
use dbConnections::db_connection::establish_connection;
use schema::accounts::dsl::*;
use diesel::prelude::*;

struct DBContext {
    context: schema::accounts::table
}

fn main() {
    let connection = establish_connection();
    let results = accounts.load::<Account>(&connection)
    .expect("Error loading accounts");

    println!("{:?}", results);



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
