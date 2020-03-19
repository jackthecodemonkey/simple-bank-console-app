use super::AccountModel::Account;

#[derive(Debug)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

impl Accounts {
    pub fn FindByAccountNo<'a>(&'a mut self, accountNo: u32) -> Result<&'a mut Account, &'static str> {
        let mut iter = self.accounts.iter_mut();
        return match iter.find(|acc| acc.no == accountNo) {
            Some(Account) => Ok(Account),
            None => Err("No matched account found"),
        };
    }

    pub fn AddAccount(&mut self, account: Account) {
        &self.accounts.push(account);
    }
}