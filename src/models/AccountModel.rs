use super::super::TextContext;

#[derive(Debug)]
pub struct Account {
    pub no: u32,
    pub name: String,
    pub deposit: i128,
}

pub enum TransactionType {
    Deposit(String),
    Withdraw(String),
    Transfer((String, String)),
}

impl TransactionType {
    fn get_transction_content(&self, content: &str) -> &str {
        match self {
            TransactionType::Deposit(ref s) => "Deposit",
            TransactionType::Withdraw(ref s) => "Withdraw",
            TransactionType::Transfer(ref s) => "Transfer",
        }
    }
}

pub trait Transaction {
    fn store_history(&mut self, type_of_transaction: &str, content: &str) -> &str;
}

impl Transaction for TextContext<'_> {
    fn store_history(&mut self, type_of_transaction: &str, content: &str) -> &str {
        "hello"
    }
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

    pub fn Stringify(&self) -> String {
        let mut s: String = String::from("\r\n");
        s.push_str(&self.no.to_string());
        s.push_str(",");
        s.push_str(&self.name.to_string());
        s.push_str(",");
        s.push_str(&self.deposit.to_string());
        s
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

    #[test]
    fn should_not_withdraw() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100);
        assert_eq!(
            account.Withdraw(101),
            Err("You don't have enough deposit to withdraw")
        );
    }

    #[test]
    fn should_determine_withdrawl() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100);
        assert_eq!(account.CanWithdraw(50), true);
        assert_eq!(account.CanWithdraw(100), true);
        assert_eq!(account.CanWithdraw(101), false);
    }

    #[test]
    fn should_convert_csv_string() {
        let mut account: Account = Account::new(1, "Jack".to_string(), 100);
        let string: String = account.Stringify();
        assert_eq!(string, String::from("\r\n1,Jack,100"));
    }
}
