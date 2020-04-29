use std::io::{self};
use std::str::FromStr;
use super::super::traits::BankServiceTrait::BankServiceTrait;
use super::super::models::AccountModel::Account;
use super::super::models::TransferModel::Transfer;

pub struct View
{
    pub service: Box<dyn BankServiceTrait>,
}

impl View {
    pub fn Display(&mut self) {
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
            match trimmed.parse::<i32>() {
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
                            vec!["i32", "string", "f64"]
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
                            vec!["i32"],
                        ) {
                            Ok(account_no) => {
                                let no: i32 = FromStr::from_str(account_no.as_str()).unwrap();
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
                            vec!["i32", "string"],
                        ) {
                            Ok(account_details) => {
                                let details: Vec<&str> = account_details.split(',').collect();
                                let no: i32 = FromStr::from_str(details[0]).unwrap();
                                let deposit: f64 = FromStr::from_str(details[1]).unwrap();
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
                            vec!["i32", "string"],
                        ) {
                            Ok(account_details) => {
                                let details: Vec<&str> = account_details.split(',').collect();
                                let no: i32 = FromStr::from_str(details[0]).unwrap();
                                let deposit: f64 = FromStr::from_str(details[1]).unwrap();
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
                            vec!["i32", "i32", "f64"],
                        ) {
                            Ok(account_details) => {
                                let v: Vec<&str> = account_details.split(',').collect();
                                let from: i32 = i32::from_str(v[0]).unwrap();
                                let to: i32 = i32::from_str(v[1]).unwrap();
                                let amount: f64 = f64::from_str(v[2]).unwrap();
                                let transfer: Transfer = Transfer { from, to, amount };
                                match self.service.Transfer(transfer) {
                                    Ok(_) => {
                                        println!("Successfully transferred");
                                        break 'transfer;
                                    }
                                    Err(e) => println!("{}", e),
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
            "i32" => {
                if let Err(_) = <i32 as FromStr>::from_str(current) {
                    invalid_arguments.push_str("Error: Invalid account number given.");
                }
            }
            "f64" => {
                if let Err(_) = <f64 as FromStr>::from_str(current) {
                    invalid_arguments.push_str(".\nError: Invalid deposit amount given.");
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
