use std::fs::File;

use super::super::models::accounts_model::Accounts;
use super::super::models::file_context::file_context;

use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct FileDBContext<'a, 'b> {
    pub context: file_context<'a>,
    pub transaction_context: file_context<'b>,
}

impl<'a, 'b> FileDBContext<'a, 'b> {
    pub fn reset_file(&mut self) -> Result<(), &str> {
        let path = Path::new(self.context.path);
        let _ = File::create(&path);
        Ok(())
    }

    pub fn rewrite_file(&mut self, accounts: &Accounts) -> Result<(), &str> {
        let remaining_accounts: String = accounts.stringify();
        let _ = self.reset_file();
        let new_file_context: file_context = file_context::new(self.context.path);
        self.context.db_context = new_file_context.db_context;
        self.context.open_options = new_file_context.open_options;
        let _ = self
            .context
            .open_options
            .write(remaining_accounts.as_bytes());
        Ok(())
    }
}
