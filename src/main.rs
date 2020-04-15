mod models;
mod services;
mod traits;

use models::AccountModel::Account;
use models::AccountsModel::Accounts;
use models::BankModel::Bank;
use models::TransactionType::TransactionType;
use models::TransferModel::Transfer;
use services::BankService::BankService;
use std::env;
// use services::SQLContext::SQLContext;
use models::FileContext::FileContext;
use services::FileDBContext::FileDBContext;
use traits::BankServiceTrait::BankServiceTrait;
use traits::Transaction::Transaction;

enum DBContext {
    File,
    DB,
}

trait ValidateCommands {
    fn validate(&self, valid_commands: &ValidCommands) -> Result<(), String>;
}

#[derive(Debug)]
struct ValidCommands {
    valid_commands: Vec<String>,
}

#[derive(Debug)]
struct Commands {
    arguments: Vec<String>,
}

impl Commands {
    fn new(commands: Vec<String>) -> Self {
        Commands {
            arguments: commands,
        }
    }
}

impl ValidateCommands for Commands {
    fn validate(&self, valid_commands: &ValidCommands) -> Result<(), String> {
        let mut invalid_commands: String = String::from("");
        for argument in self.arguments.iter() {
            if !valid_commands.valid_commands.contains(argument) {
                invalid_commands.push_str("invalid command entered: ");
                invalid_commands.push_str(&argument.as_str());
                invalid_commands.push_str("\n");
            }
        }
        if invalid_commands != "" {
            return Err(invalid_commands);
        }
        Ok(())
    }
}

fn main() {
    let arguments: Commands = Commands::new(env::args().collect());

    println!("{:?}", arguments);

    let valid_commands = ValidCommands {
        valid_commands: vec![String::from("use-file"), String::from("use-db")],
    };

    if let Err(e) = arguments.validate(&valid_commands) {
        println!("{}", e);
    }

    // simulate as if user enter text file for dbcontext
    let tempInputFromUser: DBContext = DBContext::File;

    let mut bankService = match tempInputFromUser {
        File => BankService::new(FileDBContext {
            context: FileContext::new("./src/dataSource/data.txt"),
            transaction_context: FileContext::new("./src/dataSource/transaction.txt"),
        }),
    };
    bankService.Deposit(1, 12300);
}
