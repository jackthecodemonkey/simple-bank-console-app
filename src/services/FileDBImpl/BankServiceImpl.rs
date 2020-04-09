use super::super::super::models::AccountModel::Account;
use super::super::super::models::AccountsModel::Accounts;
use super::super::super::models::TransferModel::Transfer;
use super::super::super::traits::BankServiceTrait::BankServiceTrait;
use super::super::super::traits::Transaction::Transaction;
use super::super::super::models::TransactionType::TransactionType;
use super::super::super::services::FileDBContext::FileDBContext;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::str::FromStr;

impl<'a, 'b> BankServiceTrait for FileDBContext<'a, 'b> {
    fn LoadData(&mut self) -> Accounts {
        let mut contents = String::new();
        self.context.dbContext.seek(SeekFrom::Start(0));
        self.context.dbContext.read_to_string(&mut contents);
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
    fn AddAccount(&mut self, account: Account) -> Result<Account, &str> {
        let csv_account: String = account.Stringify();
        if let Err(e) = write!(self.context.openOptions, "{}", csv_account.as_str()) {
            return Err("Failed to add a new account");
        }
        Ok(account)
    }
    fn DeleteAccount(&mut self, account_no: u32) -> &'static str {
        let mut allAccounts: Accounts = self.LoadData();
        let _ = allAccounts.DeleteAccount(account_no).unwrap();
        let _ = self.rewrite_file(&allAccounts);
        "Successfully deleted"
    }
    fn Deposit(&mut self, account_no: u32, amount: i128) -> Result<Accounts, &str> {
        let mut allAccounts: Accounts = self.LoadData();
        if let Ok(account) = allAccounts.FindByAccountNo(account_no) {
            let _ = account.Deposit(amount);
            self.log_history(TransactionType::Deposit, account.Stringify());
            let _ = self.rewrite_file(&allAccounts);
            return Ok(allAccounts);
        }
        return Err("Failed to deposit");
    }
    fn Withdraw(&mut self, account_no: u32, amount: i128) -> Result<Accounts, &str> {
        let mut allAccounts: Accounts = self.LoadData();
        if let Ok(account) = allAccounts.FindByAccountNo(account_no) {
            if account.CanWithdraw(amount) {
                let _ = account.Withdraw(amount);
                let _ = self.rewrite_file(&allAccounts);
                return Ok(allAccounts);
            }
        }
        return Err("Failed to withdraw");
    }
    fn Transfer(&mut self, transfer: Transfer) -> Result<Accounts, &str> {
        let Transfer { from, to, amount } = transfer;
        let mut allAccounts: Accounts = self.LoadData();
        if allAccounts.HasAccount(from) && allAccounts.HasAccount(to) {
            if let Ok(fromAccount) = allAccounts.FindByAccountNo(from) {
                if fromAccount.CanWithdraw(amount) {
                    let _ = self.Withdraw(from, amount);
                    return self.Deposit(to, amount);
                }
            }
        }
        Err("Failed to transfer")
    }
}
