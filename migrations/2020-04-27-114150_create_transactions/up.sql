create table transactions (
    id SERIAL PRIMARY KEY,
    no int not null,
    Transaction_type varchar not null,
    transaction_amount float not null,
    current_balance float not null
)