CREATE TYPE AccountType AS ENUM('checking', 'saving', 'credit', 'loan');
CREATE TABLE accounts (
       name TEXT NOT NULL UNIQUE PRIMARY KEY,
       account_type AccountType NOT NULL,
       date_opened timestamp with TIME ZONE NOT NULL,
       date_closed timestamp with TIME ZONE
);

CREATE TABLE periodic_budgets (
       id SERIAL PRIMARY KEY,
       start_date timestamp with TIME ZONE NOT NULL,
       end_date timestamp with TIME ZONE NOT NULL
);

CREATE TABLE one_time_budgets (
       id SERIAL PRIMARY KEY,
       description TEXT NOT NULL
);

CREATE TABLE categories (
       name TEXT UNIQUE PRIMARY KEY,
       parent TEXT
);

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
       budgeted BIGINT NOT NULL,
       transaction_type TransactionType NOT NULL,
       from_account TEXT,
       to_account TEXT,
       periodic_budget INTEGER NOT NULL,
       one_time_budget INTEGER
);

CREATE TABLE transactions (
       id SERIAL PRIMARY KEY,
       description TEXT NOT NULL,
       line_item INTEGER NOT NULL,
       transaction_type TransactionType NOT NULL,
       sending_account TEXT,
       receiving_account TEXT,
       transfer_fees BIGINT,
       receiving_entity TEXT,
       amount BIGINT NOT NULL,
       send_date timestamp with TIME ZONE NOT NULL,
       receive_date timestamp with TIME ZONE,
       corrects TEXT,
       periodic_budget INTEGER NOT NULL
);

-- Table to hold a snapshot of initial balances for a single budget for a
-- single account. The last_updated timestamp allows application logic to
-- ensure integrity of the calculation.
CREATE TABLE initial_balances (
       id SERIAL PRIMARY KEY,
       account TEXT NOT NULL,
       budget INTEGER NOT NULL,
       balance BIGINT NOT NULL,
       last_updated timestamp with TIME ZONE NOT NULL
);

-- A trigger to automatically update the last_updated timestamp column in the
-- initial balances table.
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_updated = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_initial_balance_modtime BEFORE UPDATE ON initial_balances
FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
