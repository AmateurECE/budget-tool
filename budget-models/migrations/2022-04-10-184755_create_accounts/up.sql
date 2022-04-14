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
       account SERIAL,
       periodic_budget SERIAL,
       one_time_budget SERIAL,
);
