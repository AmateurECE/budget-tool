CREATE TYPE AccountType AS ENUM('checking', 'saving', 'credit', 'loan');
CREATE TABLE accounts (
       id SERIAL PRIMARY KEY,
       name TEXT NOT NULL UNIQUE,
       account_type AccountType,
       apr DOUBLE PRECISION,
       accruing_start_date TIMESTAMP
);

CREATE TABLE periodic_budgets (
       id SERIAL PRIMARY KEY,
       start_date TIMESTAMP,
       end_date TIMESTAMP
);

CREATE TABLE one_time_budgets (
       id SERIAL PRIMARY KEY,
       description TEXT NOT NULL
);

CREATE TABLE categories (name TEXT UNIQUE PRIMARY KEY);

CREATE TABLE budget_items (
       id SERIAL PRIMARY KEY,
       description TEXT NOT NULL,
       category TEXT NOT NULL,
       budgeted MONEY,
       transaction_type TransactionType,
       from_account SERIAL,
       to_account SERIAL,
       periodic_budget SERIAL,
       one_time_budget SERIAL
);

CREATE TYPE TransactionType AS ENUM (
       'expense',
       'income',
       'transfer',
       'payment'
);

CREATE TABLE transactions (
       id SERIAL PRIMARY KEY,
       category SERIAL NOT NULL,
       line_item SERIAL NOT NULL,
       transaction_type TransactionType,
       sending_account SERIAL,
       receiving_account SERIAL,
       transfer_fees MONEY,
       receiving_entity TEXT,
       amount MONEY,
       tags TEXT[],
       send_date TIMESTAMP,
       receive_date TIMESTAMP,
       corrects SERIAL[],
       periodic_budget SERIAL
);
