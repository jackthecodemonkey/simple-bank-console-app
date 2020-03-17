mod models;

use models::Account::Account;

fn main() {
    let account = Account::new(1534556, String::from("Jack"), 0);
    println!("{:?}", account);
}
