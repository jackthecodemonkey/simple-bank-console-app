#[derive(Debug)]
pub struct Account {
    pub no: u32,
    pub name: String,
    pub deposit: i128,
}

impl Account {
    pub fn new(no: u32, name: String, deposit: i128) -> Account {
        Account { no, name, deposit }
    }

    pub fn Deposit(&mut self, amount: i128) -> &'static str {
        self.deposit = self.deposit + amount;
        return "Deposit successfully";
    }

    pub fn CanWithdraw(&mut self, amount: i128) -> bool {
        return self.deposit >= amount;
    }

    pub fn Withdraw(&mut self, amount: i128) -> Result<&'static str, &'static str> {
        return match self.deposit < amount {
            true => Err("You don't have enough deposit to withdraw"),
            false => {
                self.deposit = self.deposit - amount;
                Ok("Withdraw successfully")
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Account;
    #[test]
    fn should_deposit() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100);
        account.Deposit(500);
        assert_eq!(account.deposit, 600);

        account.Deposit(700);
        assert_eq!(account.deposit, 1300);
    }

    #[test]
    fn should_withdraw() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100);
        assert_eq!(account.Withdraw(100), Ok("Withdraw successfully"));
    }

    fn should_not_withdraw() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100);
        assert_eq!(account.Withdraw(101), Err("You don't have enough deposit to withdraw"));
    }
}
