use super::super::schema::transactions;

#[derive(Debug, Queryable, Insertable)]
pub struct Transaction {
    id: i32,
    transaction_type: String,
    transaction_amount: f64,
    current_balance: f64
} 