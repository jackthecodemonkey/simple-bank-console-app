use super::AccountModel::Account;

#[derive(Debug)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

impl Accounts {
    pub fn FindByAccountNo<'a>(
        &'a mut self,
        accountNo: u32,
    ) -> Result<&'a mut Account, &'static str> {
        let mut iter = self.accounts.iter_mut();
        return match iter.find(|acc| acc.no == accountNo) {
            Some(Account) => Ok(Account),
            None => Err("No matched account found"),
        };
    }

    pub fn HasAccount(&self, accountNo: u32) -> bool {
        let mut iter = self.accounts.iter();
        return match iter.find(|acc| acc.no == accountNo) {
            Some(_) => true,
            None => false,
        };
    }

    pub fn AddAccount(&mut self, account: Account) {
        &self.accounts.push(account);
    }

    pub fn DeleteAccount(&mut self, accountNo: u32) -> Result<(), &str> {
        if self.HasAccount(accountNo) {
            let len: usize = self.accounts.len();
            let index = self
                .accounts
                .iter()
                .position(|account| account.no == accountNo)
                .unwrap();
            self.accounts.remove(index);
            if len == self.accounts.len() {
                return Err("Failed to delete the account.");
            }
            return Ok(());
        } else {
            return Err("Account not exists");
        }
    }

    pub fn Stringify(&self) -> String {
        let mut remaining_accounts: String = String::from("");
        for account in self.accounts.iter() {
            remaining_accounts.push_str(account.Stringify().as_str());
        }
        remaining_accounts
    }

    pub fn StringifyByAccountNo(&mut self, accountNo: u32) -> Result<String, &str> {
        if self.HasAccount(accountNo) {
            if let Ok(account) = self.FindByAccountNo(accountNo) {
                return Ok(account.Stringify())
            }
        }
        Err("Account not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_find_account_by_no() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let account2 = Account::new(2, String::from("Seiko"), 5000);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);

        let mut acc = Accounts { accounts };

        if let Ok(accFound) = acc.FindByAccountNo(2) {
            assert_eq!(accFound.name, String::from("Seiko"));
        }
    }

    #[test]
    fn should_not_find_accout_by_no() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let account2 = Account::new(2, String::from("Seiko"), 5000);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);

        let mut acc = Accounts { accounts };

        if let Ok(accFound) = acc.FindByAccountNo(4) {
            assert_eq!(accFound.name, String::from("Seiko"));
        } else {
            assert!(true);
        }
    }

    #[test]
    fn can_add_account() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let account2 = Account::new(2, String::from("Seiko"), 5000);
        let mut accounts = Vec::new();
        accounts.push(account);

        let mut acc = Accounts { accounts };
        acc.AddAccount(account2);

        assert_eq!(acc.accounts.len(), 2);
    }

    #[test]
    fn should_delete_account() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let account2 = Account::new(2, String::from("Seiko"), 5000);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);
        let mut acc = Accounts { accounts };

        let _ = acc.DeleteAccount(1).unwrap();
        assert_eq!(acc.accounts.len(), 1);
    }

    #[test]
    fn should_not_delete_account() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let account2 = Account::new(2, String::from("Seiko"), 5000);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);
        let mut acc = Accounts { accounts };

        if let Err(e) = acc.DeleteAccount(10) {
            assert_eq!(e, "Account not exists");
        }
    }
}
