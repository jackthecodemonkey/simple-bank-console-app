use std::fs::File;

use super::super::models::AccountModel::Account;
use super::super::models::AccountsModel::Accounts;
use super::super::models::TransferModel::Transfer;
use super::super::traits::BankServiceTrait::BankServiceTrait;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::str::FromStr;
use std::io::SeekFrom;
use std::path::Path;

pub fn text_context_factory(path: &str) -> TextContext {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}", display),
    };

    let openOptions = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(&path)
        .unwrap();

    TextContext {
        dbContext: file,
        openOptions: openOptions,
    }
}

fn reset_file(path: &str) -> Result<(), &str> {
    let path = Path::new(path);
    let _ = File::create(&path);
    Ok(())
}

#[derive(Debug)]
pub struct TextContext {
    pub dbContext: File,
    pub openOptions: File,
}

impl BankServiceTrait for TextContext {
    fn LoadData(&mut self) -> Accounts {
        let mut contents = String::new();
        self.dbContext.seek(SeekFrom::Start(0));
        self.dbContext.read_to_string(&mut contents);
        Accounts {
            accounts: contents
                .split("\r\n")
                .filter(|&x| *&x != "")
                .map(|x| -> Account {
                    let sliced: Vec<&str> = x.split(",").collect();
                    if sliced.len() != 3 {
                        panic!("data is currupted");
                    }
                    Account {
                        no: FromStr::from_str(sliced[0]).unwrap(),
                        name: String::from(sliced[1]),
                        deposit: FromStr::from_str(sliced[2]).unwrap(),
                    }
                })
                .collect(),
        }
    }
    fn AddAccount(&mut self, account: Account) -> &'static str {
        let csv_account: String = account.Stringify();
        if let Err(e) = write!(self.openOptions, "{}", csv_account.as_str()) {
            return "Failed to add a new account";
        }
        return "Account added successfully";
    }
    fn DeleteAccount(&mut self, account_no: u32) -> &'static str {
        let mut allAccounts: Accounts = self.LoadData();
        let _ = allAccounts.DeleteAccount(account_no).unwrap();
        let remaining_accounts: String = allAccounts.Stringify();
        
        let _ = reset_file("./src/dataSource/data.txt");

        let newFileContext: TextContext = text_context_factory("./src/dataSource/data.txt");  

        self.dbContext = newFileContext.dbContext;
        self.openOptions = newFileContext.openOptions;

        self.openOptions.write(remaining_accounts.as_bytes());

        "Not implemented yet"
    }
    fn Deposit(&mut self, account_no: u32, amount: i128) -> &'static str {
        "Not implemented yet"
    }
    fn Withdraw(&mut self, account_no: u32, amount: i128) -> &'static str {
        "Not implemented yet"
    }
    fn Transfer(&mut self, transfer: Transfer) -> &'static str {
        "Not implemented yet"
    }
}
