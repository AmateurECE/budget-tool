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

CREATE TABLE line_items (
       summary TEXT NOT NULL UNIQUE PRIMARY KEY
);

CREATE TABLE line_item_instances (
       id SERIAL PRIMARY KEY,
       summary TEXT NOT NULL,
       from_account TEXT,
       to_account TEXT,
       amount BIGINT NOT NULL,
       FOREIGN KEY(summary) REFERENCES line_items(summary),
       FOREIGN KEY(from_account) REFERENCES accounts(name),
       FOREIGN KEY(to_account) REFERENCES accounts(name)
);

CREATE TABLE transactions (
       id SERIAL PRIMARY KEY,
       summary TEXT NOT NULL,
       date timestamp with TIME ZONE NOT NULL,
       from_account TEXT,
       to_account TEXT,
       amount BIGINT NOT NULL
);

CREATE TABLE real_transactions (
       id SERIAL PRIMARY KEY,
       transaction INTEGER NOT NULL,
       line_item TEXT,
       periodic_budget INTEGER,
       FOREIGN KEY(transaction) REFERENCES transactions(id),
       FOREIGN KEY(line_item) REFERENCES line_items(summary),
       FOREIGN KEY(periodic_budget) REFERENCES periodic_budgets(id)
);

CREATE TABLE planned_transactions (
       id SERIAL PRIMARY KEY,
       transaction INTEGER NOT NULL,
       line_item TEXT NOT NULL,
       periodic_budget INTEGER NOT NULL,
       FOREIGN KEY(transaction) REFERENCES transactions(id),
       FOREIGN KEY(line_item) REFERENCES line_items(summary),
       FOREIGN KEY(periodic_budget) REFERENCES periodic_budgets(id)
);
