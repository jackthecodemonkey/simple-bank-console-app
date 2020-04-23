#![feature(drain_filter)]
mod models;
mod services;
mod traits;

use models::AccountModel::Account;
use models::AccountsModel::Accounts;
use models::BankModel::Bank;
use models::TransactionType::TransactionType;
use models::TransferModel::Transfer;
use models::Commands::{ Commands, ValidCommands };
use services::BankService::BankService;
use std::env;
use services::SQLContext::SQLContext;
use models::FileContext::FileContext;
use services::FileDBContext::FileDBContext;
use traits::BankServiceTrait::BankServiceTrait;
use traits::ValidateCommands::ValidateCommands;
use traits::Transaction::Transaction;
use std::io::{self, Read};
use std::str::FromStr;

struct View<T> where T: BankServiceTrait {
    pub service: T
}

impl<T:BankServiceTrait> View<T> {
    fn Display(&mut self) {
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
                        let accounts = self.service.LoadData();
                        for account in accounts.accounts.iter() {
                            println!("{:?}", account);
                        }
                    }
                    2 => 'add_account: loop {
                        match get_account_details_from_user(
                            "enter account details with comma seprated values eg: account number, account name, deposit",
                            vec!["u32", "string", "i128"]
                        ) {
                            Ok(account_details) => {
                                let account: Account = Account::from_str(account_details);
                                match self.service.AddAccount(account) {
                                    Ok(_) => {
                                        println!("Account successfully added.");
                                        break 'add_account;
                                    }
                                    Err(e) => {
                                        println!("{}", e);
                                        println!("please try again");
                                    }
                                }
                            }
                            _ => {}
                        }
                    },
                    3 => 'delete_account: loop {
                        match get_account_details_from_user(
                            "enter account number to be deleted from system",
                            vec!["u32"],
                        ) {
                            Ok(account_no) => {
                                let no: u32 = FromStr::from_str(account_no.as_str()).unwrap();
                                self.service.DeleteAccount(no);
                                println!("Account successfully deleted.");
                                break 'delete_account;
                            }
                            _ => {}
                        }
                    },
                    4 => 'deposit: loop {
                        match get_account_details_from_user(
                            "enter account no and deposit",
                            vec!["u32", "string"],
                        ) {
                            Ok(account_details) => {
                                let details: Vec<&str> = account_details.split(',').collect();
                                let no: u32 = FromStr::from_str(details[0]).unwrap();
                                let deposit: i128 = FromStr::from_str(details[1]).unwrap();
                                match self.service.Deposit(no, deposit) {
                                    Ok(_) => {
                                        println!("Successfully deposited");
                                        break 'deposit;
                                    }
                                    Err(e) => println!("{}", e),
                                }
                            }
                            _ => {}
                        }
                    },
                    5 => 'withdraw: loop {
                        match get_account_details_from_user(
                            "enter account no and amount for withdrawal",
                            vec!["u32", "string"],
                        ) {
                            Ok(account_details) => {
                                let details: Vec<&str> = account_details.split(',').collect();
                                let no: u32 = FromStr::from_str(details[0]).unwrap();
                                let deposit: i128 = FromStr::from_str(details[1]).unwrap();
                                match self.service.Withdraw(no, deposit) {
                                    Ok(_) => {
                                        println!("Successfully withdrawn");
                                        break 'withdraw;
                                    }
                                    Err(e) => println!("{}", e),
                                }
                            }
                            _ => {}
                        }
                    },
                    6 => 'transfer: loop {
                        match get_account_details_from_user(
                            "enter your account no, the other party's account no and your amount",
                            vec!["u32", "u32", "i128"],
                        ) {
                            Ok(account_details) => {
                                let v: Vec<&str> = account_details.split(',').collect();
                                let from: u32 = u32::from_str(v[0]).unwrap();
                                let to: u32 = u32::from_str(v[1]).unwrap();
                                let amount: i128 = i128::from_str(v[2]).unwrap();
    
                                let transfer: Transfer = Transfer { from, to, amount };
                                match self.service.Transfer(transfer) {
                                    Ok(_) => {
                                        println!("Successfully transferred");
                                        break 'transfer;
                                    },
                                    Err(e) => println!("{}", e)
                                }
                            }
                            _ => {}
                        }
                    },
                    7 => {
                        break 'outer;
                    }
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
                    }
                    "n" => {
                        break 'outer;
                    }
                    _ => println!("invalid command typed {}", trimmed_next_transaction),
                }
            }
        }
    }
}

fn validate_arguments(user_arguments: &str, expected: Vec<&str>) -> Result<(), String> {
    let arguments: Vec<&str> = user_arguments.split(",").collect();
    if arguments.len() != expected.len() {
        return Err("Error: please try again.".to_string());
    }
    let mut invalid_arguments = "".to_string();
    for i in 0..expected.len() {
        let current: &str = arguments[i];
        match expected[i] {
            "u32" => {
                if let Err(_) = <u32 as FromStr>::from_str(current) {
                    invalid_arguments.push_str("Error: Invalid account number given.");
                }
            }
            "i128" => {
                if let Err(_) = <i128 as FromStr>::from_str(current) {
                    invalid_arguments.push_str(".\r\nError: Invalid deposit amount given.");
                }
            }
            _ => {}
        }
    }
    if invalid_arguments != "".to_string() {
        return Err(invalid_arguments);
    }
    Ok(())
}

fn read_input_from_user() -> String {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    input_text
}

fn get_account_details_from_user(msg: &str, expected_types: Vec<&str>) -> Result<String, ()> {
    println!("{}", msg);
    let account_str = read_input_from_user();
    let trimed = account_str.trim();
    match validate_arguments(trimed, expected_types) {
        Err(e) => {
            println!("{}", e);
            println!("pleae try again");
            return Err(());
        }
        _ => return Ok(String::from(trimed)),
    }
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

    let mut view = View {
        service: bankService
    };

    view.Display();

    // 'outer: loop {
    //     println!("****Select transaction****");
    //     println!("1. View All Accounts");
    //     println!("2. Add Account");
    //     println!("3. Delete Account");
    //     println!("4. Deposit");
    //     println!("5. Withdraw");
    //     println!("6. Transfer");
    //     println!("7. Exit");
    //     println!("****Enter a number****");
    //     let input_text = read_input_from_user();
    //     let trimmed = input_text.trim();
    //     match trimmed.parse::<u32>() {
    //         Ok(i) => match i {
    //             1 => {
    //                 let accounts = bankService.LoadData();
    //                 for account in accounts.accounts.iter() {
    //                     println!("{:?}", account);
    //                 }
    //             }
    //             2 => 'add_account: loop {
    //                 match get_account_details_from_user(
    //                     "enter account details with comma seprated values eg: account number, account name, deposit",
    //                     vec!["u32", "string", "i128"]
    //                 ) {
    //                     Ok(account_details) => {
    //                         let account: Account = Account::from_str(account_details);
    //                         match bankService.AddAccount(account) {
    //                             Ok(_) => {
    //                                 println!("Account successfully added.");
    //                                 break 'add_account;
    //                             }
    //                             Err(e) => {
    //                                 println!("{}", e);
    //                                 println!("please try again");
    //                             }
    //                         }
    //                     }
    //                     _ => {}
    //                 }
    //             },
    //             3 => 'delete_account: loop {
    //                 match get_account_details_from_user(
    //                     "enter account number to be deleted from system",
    //                     vec!["u32"],
    //                 ) {
    //                     Ok(account_no) => {
    //                         let no: u32 = FromStr::from_str(account_no.as_str()).unwrap();
    //                         bankService.DeleteAccount(no);
    //                         println!("Account successfully deleted.");
    //                         break 'delete_account;
    //                     }
    //                     _ => {}
    //                 }
    //             },
    //             4 => 'deposit: loop {
    //                 match get_account_details_from_user(
    //                     "enter account no and deposit",
    //                     vec!["u32", "string"],
    //                 ) {
    //                     Ok(account_details) => {
    //                         let details: Vec<&str> = account_details.split(',').collect();
    //                         let no: u32 = FromStr::from_str(details[0]).unwrap();
    //                         let deposit: i128 = FromStr::from_str(details[1]).unwrap();
    //                         match bankService.Deposit(no, deposit) {
    //                             Ok(_) => {
    //                                 println!("Successfully deposited");
    //                                 break 'deposit;
    //                             }
    //                             Err(e) => println!("{}", e),
    //                         }
    //                     }
    //                     _ => {}
    //                 }
    //             },
    //             5 => 'withdraw: loop {
    //                 match get_account_details_from_user(
    //                     "enter account no and amount for withdrawal",
    //                     vec!["u32", "string"],
    //                 ) {
    //                     Ok(account_details) => {
    //                         let details: Vec<&str> = account_details.split(',').collect();
    //                         let no: u32 = FromStr::from_str(details[0]).unwrap();
    //                         let deposit: i128 = FromStr::from_str(details[1]).unwrap();
    //                         match bankService.Withdraw(no, deposit) {
    //                             Ok(_) => {
    //                                 println!("Successfully withdrawn");
    //                                 break 'withdraw;
    //                             }
    //                             Err(e) => println!("{}", e),
    //                         }
    //                     }
    //                     _ => {}
    //                 }
    //             },
    //             6 => 'transfer: loop {
    //                 match get_account_details_from_user(
    //                     "enter your account no, the other party's account no and your amount",
    //                     vec!["u32", "u32", "i128"],
    //                 ) {
    //                     Ok(account_details) => {
    //                         let v: Vec<&str> = account_details.split(',').collect();
    //                         let from: u32 = u32::from_str(v[0]).unwrap();
    //                         let to: u32 = u32::from_str(v[1]).unwrap();
    //                         let amount: i128 = i128::from_str(v[2]).unwrap();

    //                         let transfer: Transfer = Transfer { from, to, amount };
    //                         match bankService.Transfer(transfer) {
    //                             Ok(_) => {
    //                                 println!("Successfully transferred");
    //                                 break 'transfer;
    //                             },
    //                             Err(e) => println!("{}", e)
    //                         }
    //                     }
    //                     _ => {}
    //                 }
    //             },
    //             7 => {
    //                 break 'outer;
    //             }
    //             _ => {
    //                 println!("invalid transaction entered {}", i);
    //                 continue 'outer;
    //             }
    //         },
    //         Err(..) => println!("invalid command typed {}", trimmed),
    //     };

    //     'inner: loop {
    //         println!("Continue other transactions? y/n");
    //         let next_transaction = read_input_from_user();
    //         let trimmed_next_transaction = next_transaction.trim();
    //         match trimmed_next_transaction {
    //             "y" => {
    //                 break 'inner;
    //             }
    //             "n" => {
    //                 break 'outer;
    //             }
    //             _ => println!("invalid command typed {}", trimmed_next_transaction),
    //         }
    //     }
    // }
}
