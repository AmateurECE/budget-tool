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
