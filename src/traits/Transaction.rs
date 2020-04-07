use super::super::models::TransactionType::TransactionType;

pub trait Transaction {
    fn log_history(&mut self, type_of_transaction: TransactionType, content: String) -> &str;
}