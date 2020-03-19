#[derive(Debug)]
pub struct Account {
    pub no: u32,
    pub name: String,
    pub deposit: i128,
}

impl Account {
    pub fn new(no: u32, name: String, deposit: i128) -> Account {
        Account {
            no,
            name,
            deposit,
        }
    }

    pub fn Deposit(&mut self, amount: i128) -> &'static str {
        self.deposit = self.deposit + amount;
        return "Deposit successfully";
    }

    fn Withdraw(mut self, amount: i128) -> Result<&'static str, &'static str> {
        return match self.deposit < amount {
            true => Err("You don't have enough deposit to withdraw"),
            false => {
                self.deposit = self.deposit - amount;
                Ok("Withdraw successfully")
            }
        };
    }
}
