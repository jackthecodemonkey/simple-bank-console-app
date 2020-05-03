use super::super::models::Transaction_type::Transaction_type;

pub trait Transaction {
    fn log_history(&mut self, type_of_transaction: Transaction_type, content: String) -> &str;
}