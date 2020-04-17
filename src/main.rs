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

use std::io::{self, Read};

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

fn read_input_from_user() -> String {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    input_text
}

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
    let file_type: String = String::from("use-file");
    let db_type: String = String::from("use-db");
    let mut bankService = match db_context_type {
        file_type => BankService::new(FileDBContext {
            context: FileContext::new("./src/dataSource/data.txt"),
            transaction_context: FileContext::new("./src/dataSource/transaction.txt"),
        }),
    };

    'outer: loop {
        println!("****Select transaction****");
        println!("1. View All Accounts");
        println!("2. Add Account");
        println!("3. Delete Account");
        println!("4. Deposit");
        println!("5. Withdraw");
        println!("6. Transfer");
        println!("7. Exit");
        println!("****Enter a number****");
        let input_text = read_input_from_user();
        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => match i {
                1 => {
                    let accounts = bankService.LoadData();
                    for account in accounts.accounts.iter() {
                        println!("{:?}", account);
                    }
                }
                2 => {}
                3 => {}
                4 => {}
                5 => {}
                6 => {}
                7 => {}
                _ => {
                    println!("invalid transaction entered {}", i);
                    continue 'outer;
                }
            },
            Err(..) => println!("invalid command typed {}", trimmed),
        };

        'inner: loop {
            println!("Continue other transactions? y/n");
            let next_transaction = read_input_from_user();
            let trimmed_next_transaction = next_transaction.trim();
    
            match trimmed_next_transaction {
                "y" => {
                    break 'inner;
                },
                "n" => {
                    break 'outer;
                },
                _ => {
                    println!("invalid command typed {}", trimmed_next_transaction)
                }
            }
        }
    }
}
