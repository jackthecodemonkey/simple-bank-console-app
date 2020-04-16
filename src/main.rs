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
    // Initialize bank service

    // let arguments: Commands = Commands::new(env::args().skip(1).collect());
    // let valid_commands = ValidCommands {
    //     valid_commands: vec![String::from("use-file"), String::from("use-db")],
    // };

    // if let Err(e) = arguments.validate(&valid_commands) {
    //     println!("{}", e);
    //     panic!("Failed to parse command line arguments")
    // }

    // let db_context_type = &arguments.arguments[0];
    // let file_type:String = String::from("use-file");
    // let db_type:String = String::from("use-db");
    // let mut bankService = match db_context_type {
    //     file_type => BankService::new(FileDBContext {
    //         context: FileContext::new("./src/dataSource/data.txt"),
    //         transaction_context: FileContext::new("./src/dataSource/transaction.txt"),
    //     }),
    // };
    
    // Ready to use bankservice 

    // Show menu to user
    // 1. Add Account
    // 2. Delete Account
    // 3. Deposit
    // 4. Withdraw
    // 5. Transfer
    // 6. Exit

    println!("****Choose transaction****");
    println!("1. Add Account");
    println!("2. Delete Account");
    println!("3. Deposit");
    println!("4. Withdraw");
    println!("5. Transfer");
    println!("6. Exit");
}
