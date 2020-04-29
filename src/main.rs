#[macro_use]
extern crate diesel;
extern crate dotenv;
mod dbConnections;
mod models;
pub mod schema;
mod services;
mod traits;
use dbConnections::db_connection::establish_connection;
use models::Commands::Commands;
use models::FileContext::FileContext;
use models::View::View;
use schema::accounts::dsl::*;
use schema::transactions::dsl::{transactions};
use services::BankService::BankService;
use services::FileDBContext::FileDBContext;
use services::SQLContext::SQLContext;
use std::env;

fn main() {
    let connection = establish_connection();
    let arguments: Commands = Commands::new(env::args().skip(1).collect());
    let db_context_type = arguments.arguments[0].as_str();

    let mut view = View {
        service: match db_context_type {
            "use-file" => Box::new(BankService::new(FileDBContext {
                context: FileContext::new("./src/dataSource/data.txt"),
                transaction_context: FileContext::new("./src/dataSource/transaction.txt"),
            })),
            "use-db" => Box::new(BankService::new(SQLContext {
                context: accounts,
                transaction_context: transactions,
                connection: connection,
            })),
            _ => panic!("Not supported db type"),
        },
    };

    view.Display();
}
