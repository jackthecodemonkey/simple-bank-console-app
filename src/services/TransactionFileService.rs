use super::super::TextContext;
use super::super::traits::Transaction::Transaction;
use super::super::models::TransactionType::TransactionType;

impl Transaction for TextContext<'_> {
    fn store_history(&mut self, transaction: TransactionType, content: &str) -> &str {
        // store whatever passed in from transaction's get_transction_content()
        // and write to file.
        println!("print from store history");
        "hello"
    }
}