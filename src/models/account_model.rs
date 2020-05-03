use std::str::FromStr;
use super::super::schema::accounts;

#[derive(Debug, Queryable, Insertable)]
pub struct Account {
    pub no: i32,
    pub name: String,
    pub deposit: f64,
}

impl Account {
    pub fn new(no: i32, name: String, deposit: f64) -> Account {
        Account { no, name, deposit }
    }

    pub fn deposit(&mut self, amount: f64) -> &'static str {
        self.deposit = self.deposit + amount;
        return "deposit successfully";
    }

    pub fn can_withdraw(&mut self, amount: f64) -> bool {
        return self.deposit >= amount;
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<&'static str, &'static str> {
        return match self.deposit < amount {
            true => Err("You don't have enough deposit to withdraw"),
            false => {
                self.deposit = self.deposit - amount;
                Ok("withdraw successfully")
            }
        };
    }

    pub fn stringify(&self) -> String {
        let mut s: String = String::from("\n");
        s.push_str(&self.no.to_string());
        s.push_str(",");
        s.push_str(&self.name.to_string());
        s.push_str(",");
        s.push_str(&self.deposit.to_string());
        s
    }

    pub fn from_str(account_details: String) -> Account {
        let split_into: Vec<&str> = account_details.split(',').collect();
        let no: i32 = FromStr::from_str(split_into[0]).unwrap();
        let name: String = split_into[1].to_string();
        let deposit: f64 = <f64 as FromStr>::from_str(split_into[2]).unwrap();
        Account {
            no,
            name,
            deposit,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Account;
    #[test]
    fn should_deposit() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100.0);
        account.deposit(500.0);
        assert_eq!(account.deposit, 600.0);

        account.deposit(700.0);
        assert_eq!(account.deposit, 1300.0);
    }

    #[test]
    fn should_withdraw() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100.0);
        assert_eq!(account.withdraw(100.0), Ok("withdraw successfully"));
    }

    #[test]
    fn should_not_withdraw() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100.0);
        assert_eq!(
            account.withdraw(101.0),
            Err("You don't have enough deposit to withdraw")
        );
    }

    #[test]
    fn should_determine_withdrawl() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100.0);
        assert_eq!(account.can_withdraw(50.0), true);
        assert_eq!(account.can_withdraw(100.0), true);
        assert_eq!(account.can_withdraw(101.0), false);
    }

    #[test]
    fn should_convert_csv_string() {
        let account: Account = Account::new(1, "Jack".to_string(), 100.0);
        let string: String = account.stringify();
        assert_eq!(string, String::from("\n1,Jack,100"));
    }
}
