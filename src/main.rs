mod models;
mod traits;
mod services;

// use models::AccountModel::Account;
// use models::AccountsModel::Accounts;
// use models::BankModel::Bank;
// use models::TransferModel::Transfer;

// use services::BankService::BankService;
// use services::SQLContext::SQLContext;
use services::TextContext::TextContext;
use traits::BankServiceTrait::BankServiceTrait;

use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("./src/dataSource/data.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}",display), 
    };

    let mut textContext: TextContext = TextContext {
        dbContext: file,
    };

    textContext.LoadData();

    // let bankService: BankService<SQLContext> = BankService {
    //     dbContext: sqlContext,
    // };

    // println!("{:?}", bankService);

    // let account = Account::new(1534556, String::from("Jack"), 1000);
    // let account2 = Account::new(4534556, String::from("Seiko"), 5000);

    // let accounts = Accounts { accounts: vec![] };

    // let mut bank = Bank::new(accounts);

    // bank.AddAccount(account);
    // bank.AddAccount(account2);

    // bank.Delete_account(151234556);

    // println!("{:?}", bank);
}
