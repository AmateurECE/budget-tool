# Budgeting and Financial Tracking Tool

# Database Management in Development

Use of the `sqlx-cli` tool can greatly ease the management of databases in
development. Start by bringing up the database server:

```bash-session
$ cd budget-service && docker-compose up
```

The environment variable `DATABASE_URL` must be set for all invocations of the
`sqlx-cli` tool. For most development, this variable should probably be set to
`postgres://postgres:example@localhost:5432/budgets`.

To create the database:

```bash
# Drop the existing database and create a new one
sqlx database drop && sqlx database create

# Run all of our migrations
sqlx migrate --source migrations/ run
```
