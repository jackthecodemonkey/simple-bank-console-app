#[allow(dead_code)]
use super::account_model::Account;

#[derive(Debug)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

impl Accounts {
    pub fn find_by_account_no<'a>(
        &'a mut self,
        account_no: i32,
    ) -> Result<&'a mut Account, &'static str> {
        let mut iter = self.accounts.iter_mut();
        return match iter.find(|acc| acc.no == account_no) {
            Some(account) => Ok(account),
            None => Err("No matched account found"),
        };
    }

    pub fn has_account(&self, account_no: i32) -> bool {
        let mut iter = self.accounts.iter();
        return match iter.find(|acc| acc.no == account_no) {
            Some(_) => true,
            None => false,
        };
    }

    pub fn add_account(&mut self, account: Account) {
        &self.accounts.push(account);
    }

    pub fn delete_account(&mut self, account_no: i32) -> Result<(), &str> {
        if self.has_account(account_no) {
            let len: usize = self.accounts.len();
            let index = self
                .accounts
                .iter()
                .position(|account| account.no == account_no)
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

    pub fn stringify(&self) -> String {
        let mut remaining_accounts: String = String::from("");
        for account in self.accounts.iter() {
            remaining_accounts.push_str(account.stringify().as_str());
        }
        remaining_accounts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_find_account_by_no() {
        let account = Account::new(1, String::from("Jack"), 1000.0);
        let account2 = Account::new(2, String::from("Seiko"), 5000.0);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);

        let mut acc = Accounts { accounts };

        if let Ok(acc_found) = acc.find_by_account_no(2) {
            assert_eq!(acc_found.name, String::from("Seiko"));
        }
    }

    #[test]
    fn should_not_find_accout_by_no() {
        let account = Account::new(1, String::from("Jack"), 1000.0);
        let account2 = Account::new(2, String::from("Seiko"), 5000.0);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);

        let mut acc = Accounts { accounts };

        if let Ok(acc_found) = acc.find_by_account_no(4) {
            assert_eq!(acc_found.name, String::from("Seiko"));
        } else {
            assert!(true);
        }
    }

    #[test]
    fn can_add_account() {
        let account = Account::new(1, String::from("Jack"), 1000.0);
        let account2 = Account::new(2, String::from("Seiko"), 5000.0);
        let mut accounts = Vec::new();
        accounts.push(account);

        let mut acc = Accounts { accounts };
        acc.add_account(account2);

        assert_eq!(acc.accounts.len(), 2);
    }

    #[test]
    fn should_delete_account() {
        let account = Account::new(1, String::from("Jack"), 1000.0);
        let account2 = Account::new(2, String::from("Seiko"), 5000.0);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);
        let mut acc = Accounts { accounts };

        let _ = acc.delete_account(1).unwrap();
        assert_eq!(acc.accounts.len(), 1);
    }

    #[test]
    fn should_not_delete_account() {
        let account = Account::new(1, String::from("Jack"), 1000.0);
        let account2 = Account::new(2, String::from("Seiko"), 5000.0);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);
        let mut acc = Accounts { accounts };

        if let Err(e) = acc.delete_account(10) {
            assert_eq!(e, "Account not exists");
        }
    }
}
