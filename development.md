* Working on File IO
    0. Fix a bug: account number should be unique when adding : Done
    1. Implement load data from CSV and create Account struct -> return Accounts models : Done
    2. Implement AddAccount, pass account as a param, make it comma separated sting then write(amend) to file : Done
    3. Refactor the AddAccount : pull the logic for making string out of Account : Done
    4. Implement DeleteAccount, pass account no as a param, find from the file and remove it
       TODO: Continue coding for DeleteAccount.
             1. loop through accounts, call stringify and concat : DONE
             2. clean up txt file : DONE
                TODO:: refactor code of DeleteAccount a bit, write some unit testing : DONE
             3. and write new string : DONE

    5. Add Bank model to db context : DONE
       update AddAccount and DeleteAccount to use Bank model business logic : DONE
      
   * Think about how we combine controller & services & models alltogether
     BankModel might be a redundant
     Controller can get data from BankService and pass it to view thats all.

   TODO:: Implement a method to store all transaction history per account?
   Transactions: Deposit, Withdraw, Transfer

      1. Update BankServiceTrait.rs and BankService.rs so that it gets another service(TransactionContext either file or db) and it calls "store_history" from methods of BankService

    6. Implement BankService 
       1. BankService will get a dbcontext service between TextContext and DbContext : DONE
         TODO: 1. Refactor TextContext.rs, split into multiple files with smaller file sizes
               2. Continue implementing transaction context

       2. This service will be injected to a controller to consume

    7. Implement controller 
       1. use bankservice for transactions
             
* Add a mechanism to switch between file and db context ( factory pattern? )
 