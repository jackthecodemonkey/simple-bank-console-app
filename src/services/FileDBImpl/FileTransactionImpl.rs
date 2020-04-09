use super::super::super::services::FileDBContext::FileDBContext;
use super::super::super::models::TransactionType::TransactionType;
use super::super::super::traits::Transaction::Transaction;

impl<'a, 'b> Transaction for FileDBContext<'a, 'b> {
    fn log_history(&mut self, type_of_transaction: TransactionType, content: String) -> &str {
        let string: String = type_of_transaction.get_transction_content(content);
        println!("{:?}",string);
        "Not implemented"
    }
}