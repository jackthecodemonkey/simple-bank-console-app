mod models;

use models::AccountModel::Account;
use models::AccountsModel::Accounts;
use models::BankModel::Bank;

fn main() {
    let account = Account::new(1534556, String::from("Jack"), 1000);
    let account2 = Account::new(4534556, String::from("Seiko"), 5000);

    let accounts = Accounts { accounts: vec![] };

    let mut bank = Bank::new(accounts);

    bank.AddAccount(account);
    bank.AddAccount(account2);

    println!("{:?}", bank);

}
