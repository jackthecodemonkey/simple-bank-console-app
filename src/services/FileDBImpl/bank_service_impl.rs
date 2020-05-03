use super::super::super::models::account_model::Account;
use super::super::super::models::accounts_model::Accounts;
use super::super::super::models::Transaction_type::Transaction_type;
use super::super::super::models::transfer_model::Transfer;
use super::super::super::services::FileDBContext::FileDBContext;
use super::super::super::traits::BankServiceTrait::bank_service_trait;
use super::super::super::traits::Transaction::Transaction;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::str::FromStr;

impl<'a, 'b> bank_service_trait for FileDBContext<'a, 'b> {
    fn load_data(&mut self) -> Accounts {
        let mut contents = String::new();
        let _ = self.context.db_context.seek(SeekFrom::Start(0));
        let _ = self.context.db_context.read_to_string(&mut contents);
        Accounts {
            accounts: contents
                .split("\n")
                .filter(|&x| *&x != "")
                .map(|x| -> Account {
                    let sliced: Vec<&str> = x.split(",").collect();
                    println!("{:?}", sliced);
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
    fn add_account(&mut self, account: Account) -> Result<Account, &str> {
        let all_accounts: Accounts = self.load_data();
        match all_accounts.has_account(account.no) {
            true => {
                return Err("Account already exists.");
            }
            false => {
                let csv_account: String = account.stringify();
                if let Err(e) = write!(self.context.open_options, "{}", csv_account.as_str()) {
                    return Err("Failed to add a new account");
                }
                Ok(account)
            }
        }
    }
    fn delete_account(&mut self, account_no: i32) -> &'static str {
        let mut all_accounts: Accounts = self.load_data();
        let _ = all_accounts.delete_account(account_no).unwrap();
        let _ = self.rewrite_file(&all_accounts);
        "Successfully deleted"
    }
    fn deposit(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        let mut all_accounts: Accounts = self.load_data();
        if let Ok(account) = all_accounts.find_by_account_no(account_no) {
            let _ = account.deposit(amount);
            self.log_history(Transaction_type::Deposit, account.stringify());
            let _ = self.rewrite_file(&all_accounts);
            return Ok(all_accounts);
        }
        return Err("Failed to deposit");
    }
    fn withdraw(&mut self, account_no: i32, amount: f64) -> Result<Accounts, &str> {
        let mut all_accounts: Accounts = self.load_data();
        if let Ok(account) = all_accounts.find_by_account_no(account_no) {
            if account.can_withdraw(amount) {
                self.log_history(Transaction_type::Withdraw, account.stringify());
                let _ = account.withdraw(amount);
                let _ = self.rewrite_file(&all_accounts);
                return Ok(all_accounts);
            }
        }
        return Err("Failed to withdraw");
    }
    fn transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str> {
        let Transfer { from, to, amount } = transfer;
        let mut all_accounts: Accounts = self.load_data();
        if all_accounts.has_account(from) && all_accounts.has_account(to) {
            if let Ok(from_account) = all_accounts.find_by_account_no(from) {
                if from_account.can_withdraw(amount) {
                    self.log_history(Transaction_type::Transfer, String::from("\n"));
                    let _ = self.withdraw(from, amount);
                    return self.deposit(to, amount);
                }
            }
        }
        Err("Failed to transfer")
    }
}
