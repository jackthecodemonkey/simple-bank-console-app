mod models;
mod services;
mod traits;

use models::AccountModel::Account;
use models::AccountsModel::Accounts;
use models::BankModel::Bank;
use models::TransferModel::Transfer;

// use services::BankService::BankService;
// use services::SQLContext::SQLContext;
use services::TextContext::*;
use traits::BankServiceTrait::BankServiceTrait;

fn main() {
    let mut bank = Bank {
        accounts: Accounts {
            accounts: vec![]
        }
    };

    let mut textContext: TextContext = text_context_factory("./src/dataSource/data.txt", &mut bank);

    textContext.LoadData();

    // let acc: Account = Account {
    //     no: 7,
    //     name: String::from("VicVic"),
    //     deposit: 8900,
    // };

    // textContext.AddAccount(acc);

    // textContext.DeleteAccount(5);

    textContext.Deposit(2, 4000);

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
