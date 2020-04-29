use super::super::schema::transactions;

#[derive(Debug, Queryable, Insertable, Default)]
pub struct Transaction {
    pub id: i32,
    pub no: i32,
    pub transaction_type: String,
    pub transaction_amount: f64,
    pub current_balance: f64,
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub no: i32,
    pub transaction_type: String,
    pub transaction_amount: f64,
    pub current_balance: f64,
}
