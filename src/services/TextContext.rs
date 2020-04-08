use std::fs::File;

use super::super::models::AccountModel::Account;
use super::super::models::AccountsModel::Accounts;
use super::super::models::TransferModel::Transfer;
use super::super::traits::BankServiceTrait::BankServiceTrait;
use super::super::traits::Transaction::Transaction;
use super::super::models::TransactionType::TransactionType;


use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct FileDBContext<'a, 'b> {
    pub context: TextContext<'a>,
    pub transaction_context: TextContext<'b>,
}

#[derive(Debug)]
pub struct TextContext<'a> {
    pub dbContext: File,
    pub openOptions: File,
    pub path: &'a str,
}

impl TextContext<'_> {
    pub fn new(path: &str) -> TextContext {
        let file_path = Path::new(path);
        let display = file_path.display();
        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => panic!("couldn't open {}", display),
        };
        let openOptions = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&file_path)
            .unwrap();
        TextContext {
            dbContext: file,
            openOptions: openOptions,
            path,
        }
    }
}

impl<'a, 'b> Transaction for FileDBContext<'a, 'b> {
    fn log_history(&mut self, type_of_transaction: TransactionType, content: String) -> &str {
        let string: String = type_of_transaction.get_transction_content(content);
        println!("{:?}",string);
        "Not implemented"
    }
}

impl<'a, 'b> FileDBContext<'a, 'b> {
    fn reset_file(&mut self) -> Result<(), &str> {
        let path = Path::new(self.context.path);
        let _ = File::create(&path);
        Ok(())
    }

    fn rewrite_file(&mut self, accounts: &Accounts) -> Result<(), &str> {
        let remaining_accounts: String = accounts.Stringify();
        let _ = self.reset_file();
        let newFileContext: TextContext = TextContext::new(self.context.path);
        self.context.dbContext = newFileContext.dbContext;
        self.context.openOptions = newFileContext.openOptions;
        let _ = self
            .context
            .openOptions
            .write(remaining_accounts.as_bytes());
        Ok(())
    }
}

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
                if (fromAccount.CanWithdraw(amount)) {
                    let _ = self.Withdraw(from, amount);
                    return self.Deposit(to, amount);
                }
            }
        }
        Err("Failed to transfer")
    }
}
