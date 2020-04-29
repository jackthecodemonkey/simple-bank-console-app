use super::super::schema::transactions;

#[derive(Debug, Queryable, Insertable, Default)]
pub struct Transaction {
    pub no: i32,
    pub transaction_type: String,
    pub transaction_amount: f64,
    pub current_balance: f64
} 