table! {
    accounts (no) {
        no -> Int4,
        name -> Varchar,
        deposit -> Float8,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        transaction_type -> Varchar,
        transaction_amount -> Float8,
        current_balance -> Float8,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
