CREATE TYPE AccountType AS ENUM('checking', 'saving', 'credit', 'loan');
CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    account_type AccountType,
    apr DOUBLE PRECISION,
    accruing_start_date TIMESTAMP
);
