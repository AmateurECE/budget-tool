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

CREATE TYPE TransactionType AS ENUM (
       'expense',
       'income',
       'transfer',
       'payment'
);

CREATE TABLE budget_items (
       id SERIAL PRIMARY KEY,
       description TEXT NOT NULL,
       category TEXT NOT NULL,
       budgeted MONEY,
       transaction_type TransactionType,
       from_account INTEGER,
       to_account INTEGER,
       periodic_budget INTEGER,
       one_time_budget INTEGER
);

CREATE TABLE transactions (
       id SERIAL PRIMARY KEY,
       category INTEGER NOT NULL,
       line_item INTEGER NOT NULL,
       transaction_type TransactionType,
       sending_account INTEGER,
       receiving_account INTEGER,
       transfer_fees MONEY,
       receiving_entity TEXT,
       amount MONEY,
       tags TEXT[],
       send_date TIMESTAMP,
       receive_date TIMESTAMP,
       corrects INTEGER[],
       periodic_budget INTEGER
);

CREATE TABLE initial_balances (
       id SERIAL PRIMARY KEY,
       account INTEGER,
       budget INTEGER,
       balance MONEY,
       last_updated TIMESTAMP
);
