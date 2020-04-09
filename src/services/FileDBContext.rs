use std::fs::File;

use super::super::models::AccountModel::Account;
use super::super::models::AccountsModel::Accounts;
use super::super::models::TransferModel::Transfer;
use super::super::models::FileContext::FileContext;
use super::super::traits::BankServiceTrait::BankServiceTrait;
use super::super::traits::Transaction::Transaction;
use super::super::models::TransactionType::TransactionType;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct FileDBContext<'a, 'b> {
    pub context: FileContext<'a>,
    pub transaction_context: FileContext<'b>,
}

impl<'a, 'b> FileDBContext<'a, 'b> {
    pub fn reset_file(&mut self) -> Result<(), &str> {
        let path = Path::new(self.context.path);
        let _ = File::create(&path);
        Ok(())
    }

    pub fn rewrite_file(&mut self, accounts: &Accounts) -> Result<(), &str> {
        let remaining_accounts: String = accounts.Stringify();
        let _ = self.reset_file();
        let newFileContext: FileContext = FileContext::new(self.context.path);
        self.context.dbContext = newFileContext.dbContext;
        self.context.openOptions = newFileContext.openOptions;
        let _ = self
            .context
            .openOptions
            .write(remaining_accounts.as_bytes());
        Ok(())
    }
}
