use std::io::Write;
use super::super::super::services::FileDBContext::FileDBContext;
use super::super::super::models::Transaction_type::Transaction_type;
use super::super::super::traits::Transaction::Transaction;

impl<'a, 'b> Transaction for FileDBContext<'a, 'b> {
    fn log_history(&mut self, type_of_transaction: Transaction_type, content: String) -> &str {
        let string: String = type_of_transaction.get_transction_content(content);
        if let Err(_) = write!(self.transaction_context.open_options, "{}", string.as_str()) {
            return "Failed to log transaction"
        }
        "Not implemented"
    }
}