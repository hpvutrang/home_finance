CREATE TABLE IF NOT EXISTS account_families (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) UNIQUE
);

INSERT INTO account_families (name) VALUES 
    ('Asset'),
    ('Liability'),
    ('Equity'),
    ('Income'),
    ('Expense');

CREATE TABLE IF NOT EXISTS accounts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256),
    family INTEGER NOT NULL REFERENCES account_families(id) ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS entries 
(
    id SERIAL PRIMARY KEY,
    description VARCHAR(1024) UNIQUE NOT NULL,
    event_date TIMESTAMPTZ NOT NULL,
    amount NUMERIC(20, 2) NOT NULL CHECK (amount > 0.0),
    credit INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    debit INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT
);

-- Side notes:
-- start_date or end_date == null => no start or end. appearence by first day of the frequency (daily -> 0:00, weekly -> monday 0:00, monthly -> 1st : moneday 0:0, etc.)
CREATE TYPE FREQUENCE AS ENUM ('daily', 'weekly', 'monthly', 'yearly');
CREATE TABLE IF NOT EXISTS recurrences
(
    id SERIAL PRIMARY KEY,
    description VARCHAR(1024) UNIQUE NOT NULL,
    credit INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    debit INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    start_date DATE,
    end_date DATE, -- Inclusive
    amount NUMERIC(20, 2) NOT NULL CHECK (amount > 0.0),
    frequence FREQUENCE
);

CREATE INDEX ON accounts(family);
CREATE INDEX ON entries(credit);
CREATE INDEX ON entries(debit);
CREATE INDEX ON recurrences(credit);
CREATE INDEX ON recurrences(debit);

-- READ ONLY
CREATE OR REPLACE RULE prohibit_insert AS 
ON INSERT
TO account_families
DO INSTEAD NOTHING; 

CREATE OR REPLACE RULE prohibit_update AS 
ON UPDATE
TO account_families
DO INSTEAD NOTHING; 

CREATE OR REPLACE RULE prohibit_delete AS 
ON DELETE 
TO account_families 
DO INSTEAD NOTHING; 
---

CREATE VIEW account_ledgers(
    account_id,
    entry_id,
    amount
) AS
    SELECT
        entries.credit,
        entries.id,
        entries.amount
    FROM entries
    UNION ALL
    SELECT
        entries.debit,
        entries.id,
        (0.0 - entries.amount)
    FROM entries;

CREATE MATERIALIZED VIEW account_balances(
    -- Materialized so financial reports run fast
    id, -- INTEGER REFERENCES accounts(id) not
    balance -- NUMERIC NOT NULL
) AS
    SELECT
        accounts.id,
        COALESCE(sum(account_ledgers.amount), 0.0)
    FROM
        accounts
        LEFT OUTER JOIN account_ledgers
        ON accounts.id = account_ledgers.account_id
    GROUP BY accounts.id;

CREATE UNIQUE INDEX ON account_balances(id);

CREATE FUNCTION update_balances() RETURNS TRIGGER AS $$
BEGIN
    REFRESH MATERIALIZED VIEW account_balances;
    RETURN NULL;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_fix_balance_entries
AFTER INSERT
OR UPDATE OF amount, credit, debit
OR DELETE OR TRUNCATE
ON entries
FOR EACH STATEMENT
    EXECUTE PROCEDURE update_balances();

CREATE TRIGGER trigger_fix_balance_accounts
AFTER INSERT
OR UPDATE OF id
OR DELETE OR TRUNCATE
ON accounts
FOR EACH STATEMENT
    EXECUTE PROCEDURE update_balances();
