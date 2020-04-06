use super::super::models::TransactionType::TransactionType;

pub trait Transaction {
    fn store_history(&mut self, type_of_transaction: TransactionType, content: &str) -> &str;
}