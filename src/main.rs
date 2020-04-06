mod models;
mod services;
mod traits;

use models::AccountModel::Account;
use models::AccountsModel::Accounts;
use models::BankModel::Bank;
use models::TransferModel::Transfer;
use models::TransactionType::TransactionType;
use services::BankService::BankService;
// use services::SQLContext::SQLContext;
use services::TextContext::{ TextContext };
use traits::BankServiceTrait::BankServiceTrait;
use traits::Transaction::Transaction;

enum DBContext {
    File,
    DB,
}

fn main() {
    // simulate as if user enter text file for dbcontext
    let tempInputFromUser: DBContext = DBContext::File;

    let mut bankService = match tempInputFromUser {
        File => BankService::new(TextContext::new("./src/dataSource/data.txt")),
        DB => BankService::new(TextContext::new("./src/dataSource/data.txt")),
    };

    let mut a = TextContext::new("./src/dataSource/data.txt");

    a.store_history(TransactionType::Deposit(String::from("Deposit")), "hello");

    let deposit = TransactionType::Deposit(String::from("deposit"));

    deposit.get_transction_content("Hello deposit");

    // println!("{:?}", bankService.LoadData());

    // let mut textContext = TextContext::new("./src/dataSource/data.txt");


    // println!("{:?}",bankService.LoadData());

    // let acc: Account = Account {
    //     no: 17,
    //     name: String::from("VicVic"),
    //     deposit: 8900,
    // };

    // textContext.AddAccount(acc);

    // textContext.DeleteAccount(17);

    // textContext.Transfer(Transfer {
    //     from: 2,
    //     to: 6,
    //     amount: 50,
    // });

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
