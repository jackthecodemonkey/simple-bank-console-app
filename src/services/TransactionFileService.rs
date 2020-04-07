use super::super::TextContext;
use super::super::traits::Transaction::Transaction;
use super::super::models::TransactionType::TransactionType;

impl Transaction for TextContext<'_> {
    fn log_history(&mut self, transaction: TransactionType, content: String) -> &str {
        // store whatever passed in from transaction's get_transction_content()
        // and write to file.
        let string: String = transaction.get_transction_content(content);
        println!("print from store history");
        println!("{:}",string);
        "hello"
    }
}