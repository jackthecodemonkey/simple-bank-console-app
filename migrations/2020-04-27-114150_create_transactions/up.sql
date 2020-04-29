create table transactions (
    no int not null PRIMARY KEY,
    transaction_type varchar not null,
    transaction_amount float not null,
    current_balance float not null
)