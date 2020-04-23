#![feature(drain_filter)]
mod models;
mod services;
mod traits;
use models::Commands::{ Commands, ValidCommands };
use services::BankService::BankService;
use std::env;
use models::FileContext::FileContext;
use services::FileDBContext::FileDBContext;
use traits::ValidateCommands::ValidateCommands;
use models::View::View;

fn main() {
    let arguments: Commands = Commands::new(env::args().skip(1).collect());
    let valid_commands = ValidCommands {
        valid_commands: vec![String::from("use-file"), String::from("use-db")],
    };

    if let Err(e) = arguments.validate(&valid_commands) {
        println!("{}", e);
        panic!("Failed to parse command line arguments")
    }

    let db_context_type = &arguments.arguments[0];
    let _: String = String::from("use-file");
    let _: String = String::from("use-db");
    let bank_service = match db_context_type {
        _ => BankService::new(FileDBContext {
            context: FileContext::new("./src/dataSource/data.txt"),
            transaction_context: FileContext::new("./src/dataSource/transaction.txt"),
        }),
    };

    let mut view = View {
        service: bank_service
    };

    view.Display();
}
