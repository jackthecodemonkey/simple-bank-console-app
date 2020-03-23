mod models;
mod traits;
mod services;

use models::AccountModel::Account;
use models::AccountsModel::Accounts;
use models::BankModel::Bank;
use models::TransferModel::Transfer;

use services::BankService::BankService;
use services::SQLContext::SQLContext;
use services::TextContext::TextContext;

fn main() {
    let sqlContext: SQLContext = SQLContext { dbContext: 10 };
    let textContext: TextContext = TextContext {
        dbContext: String::from("TEXT"),
    };

    let bankService: BankService<SQLContext> = BankService {
        dbContext: sqlContext,
    };

    println!("{:?}", bankService);

    // let account = Account::new(1534556, String::from("Jack"), 1000);
    // let account2 = Account::new(4534556, String::from("Seiko"), 5000);

    // let accounts = Accounts { accounts: vec![] };

    // let mut bank = Bank::new(accounts);

    // bank.AddAccount(account);
    // bank.AddAccount(account2);

    // bank.Delete_account(151234556);

    // println!("{:?}", bank);
}
