* Working on File IO
    0. Fix a bug: account number should be unique when adding : Done
    1. Implement load data from CSV and create Account struct -> return Accounts models : Done
    2. Implement AddAccount, pass account as a param, make it comma separated sting then write(amend) to file : Done
    3. Refactor the AddAccount : pull the logic for making string out of Account
    4. Implement DeleteAccount, pass account no as a param, find from the file and remove it
* Add a mechanism to switch between file and db context ( factory pattern? )
 