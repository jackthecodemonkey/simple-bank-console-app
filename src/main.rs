#[macro_use]
extern crate diesel;
extern crate dotenv;
mod db_connections;
mod models;
pub mod schema;
mod services;
mod traits;
use db_connections::db_connection::establish_connection;
use models::commands::commands;
use models::file_context::file_context;
use models::view::View;
use schema::accounts::dsl::*;
use schema::transactions::dsl::{transactions};
use services::BankService::BankService;
use services::FileDBContext::FileDBContext;
use services::SQLContext::SQLContext;
use std::env;

fn main() {
    let connection = establish_connection();
    let arguments: commands = commands::new(env::args().skip(1).collect());
    let db_context_type = arguments.arguments[0].as_str();

    let mut view = View {
        service: match db_context_type {
            "use-file" => Box::new(BankService::new(FileDBContext {
                context: file_context::new("./src/dataSource/data.txt"),
                transaction_context: file_context::new("./src/dataSource/transaction.txt"),
            })),
            "use-db" => Box::new(BankService::new(SQLContext {
                context: accounts,
                transaction_context: transactions,
                connection: connection,
            })),
            _ => panic!("Not supported db type"),
        },
    };

    view.display();
}
