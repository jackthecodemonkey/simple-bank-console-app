use super::AccountModel::Account;
use super::AccountsModel::Accounts;
use super::TransferModel::Transfer;

#[derive(Debug)]
pub struct Bank {
    pub accounts: Accounts,
}

impl Bank {
    pub fn new(accounts: Accounts) -> Self {
        Bank { accounts }
    }
}

impl Bank {
    pub fn AddAccount(&mut self, account: Account) {
        &self.accounts.AddAccount(account);
    }

    pub fn FindByAccountNo<'a>(
        &'a mut self,
        accountNo: u32,
    ) -> Result<&'a mut Account, &'static str> {
        self.accounts.FindByAccountNo(accountNo)
    }

    pub fn Deposit(&mut self, accountNo: u32, amount: i128) -> Result<&Account, &'static str> {
        let account = self.FindByAccountNo(accountNo)?;
        account.Deposit(amount);
        Ok(account)
    }

    pub fn Transfer(&mut self, transfer: Transfer) -> Result<&'static str, &'static str> {
        let fromAccount = self.FindByAccountNo(transfer.from)?;
        match fromAccount.CanWithdraw(transfer.amount) {
            true => {
                fromAccount.Withdraw(transfer.amount);
                self.Deposit(transfer.to, transfer.amount);
                return Ok("Transfer successfully");
            }
            false => {
                return Err("Your account doesn't have enough depost to transfer");
            }
        }
    }

    pub fn Delete_account(&mut self, no: u32) -> Result<&'static str, &'static str> {
        let original_accounts_len: usize = self.accounts.accounts.len();
        self.accounts.accounts.retain(|account| account.no != no);

        return match original_accounts_len < self.accounts.accounts.len() {
            true => Ok("Account deleted successfully"),
            false => Err("Unable to find the account."),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_account() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let account2 = Account::new(2, String::from("Seiko"), 5000);
        let mut accounts = Vec::new();
        accounts.push(account);

        let acc = Accounts { accounts };
        let mut bank = Bank { accounts: acc };

        bank.AddAccount(account2);
        assert_eq!(bank.accounts.accounts.len(), 2);
    }

    #[test]
    fn find_by_no() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let mut accounts = Vec::new();
        accounts.push(account);

        let acc = Accounts { accounts };
        let mut bank = Bank { accounts: acc };
        if let Ok(account) = bank.FindByAccountNo(1) {
            assert_eq!(account.name, String::from("Jack"));
        }
    }

    #[test]
    fn should_deposit() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let mut accounts = Vec::new();
        accounts.push(account);

        let acc = Accounts { accounts };
        let mut bank = Bank { accounts: acc };

        if let Ok(jack_account) = bank.Deposit(1, 500) {
            assert_eq!(jack_account.deposit, 1500);
        }
    }

    #[test]
    fn should_transfer() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let account2 = Account::new(2, String::from("Seiko"), 5000);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);

        let acc = Accounts { accounts };
        let mut bank = Bank { accounts: acc };

        let transfer_from_jack_to_seiko = Transfer {
            from: 1,
            to: 2,
            amount: 750,
        };

        bank.Transfer(transfer_from_jack_to_seiko);

        if let Ok(seiko_account) = bank.FindByAccountNo(2) {
            assert_eq!(seiko_account.deposit, 5750);
        }
    }

    #[test]
    fn should_delete_account() {
        let account = Account::new(1, String::from("Jack"), 1000);
        let account2 = Account::new(2, String::from("Seiko"), 5000);
        let account3 = Account::new(3, String::from("Mia"), 5000);
        let mut accounts = Vec::new();
        accounts.push(account);
        accounts.push(account2);

        let acc = Accounts { accounts };
        let mut bank = Bank { accounts: acc };

        bank.Delete_account(1);
        assert_eq!(bank.accounts.accounts.len(), 1);

        bank.AddAccount(account3);
        assert_eq!(bank.accounts.accounts.len(), 2);

        bank.Delete_account(123);
        assert_eq!(bank.accounts.accounts.len(), 2);
    }
}
