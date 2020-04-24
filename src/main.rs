mod models;
mod services;
mod traits;
use models::Commands::Commands;
use models::FileContext::FileContext;
use models::View::View;
use services::BankService::BankService;
use services::FileDBContext::FileDBContext;
use std::env;

fn main() {
    let arguments: Commands = Commands::new(env::args().skip(1).collect());
    let db_context_type = arguments.arguments[0].as_str();
    let bank_service = match db_context_type {
        "use-file" => BankService::new(FileDBContext {
            context: FileContext::new("./src/dataSource/data.txt"),
            transaction_context: FileContext::new("./src/dataSource/transaction.txt"),
        }),
        "use-db" => BankService::new(FileDBContext {
            context: FileContext::new("./src/dataSource/data.txt"),
            transaction_context: FileContext::new("./src/dataSource/transaction.txt"),
        }),
        _ => panic!("Not supported db type"),
    };

    let mut view = View {
        service: bank_service,
    };

    view.Display();
}
