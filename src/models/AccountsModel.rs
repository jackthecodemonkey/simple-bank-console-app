use super::AccountModel::Account;

#[derive(Debug)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

impl Accounts {
    pub fn FindByAccountNo<'a>(&'a mut self, accountNo: u32) -> Result<&'a Account, String> {
        let mut iter = self.accounts.iter();
        return match iter.find(|acc| acc.no == accountNo) {
            Some(Account) => Ok(Account),
            None => Err(String::from("No matched account found")),
        };
    }

    pub fn AddAccount(&mut self, account: Account) {
        &self.accounts.push(account);
    }
}
