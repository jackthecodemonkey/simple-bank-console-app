use super::AccountModel::Account;
use super::AccountsModel::Accounts;
use super::TransferModel::Transfer;

#[derive(Debug)]
pub struct Bank {
    pub accounts: Accounts,
}

impl Bank {
    pub fn new(accounts: Accounts) -> Self {
        Bank {
            accounts
        }
    }
}

impl Bank {
    pub fn AddAccount(&mut self, account: Account) {
        &self.accounts.AddAccount(account);
    }

    pub fn FindByAccountNo<'a>(&'a mut self, accountNo: u32) -> Result<&'a Account, String> {
        self.accounts.FindByAccountNo(accountNo)
    }
}
