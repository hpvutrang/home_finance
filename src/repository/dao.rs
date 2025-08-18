use std::error::Error;

use crate::repository::dto;

pub(super) struct Dao {
    pub pool: deadpool_postgres::Pool,
}

pub(super) fn new(pool: deadpool_postgres::Pool) -> Dao {
    Dao {
        pool: pool,
    }
}

impl Dao {
    pub(super) async fn insert_account(
        &self,
        account: &dto::Account,
    ) -> Result<i32, Box<dyn Error>> {
        let query = "INSERT INTO accounts (name, family) VALUES ($1, $2) RETURNING id";
        let client = self.pool.get().await?;
        let row = client
            .query_one(query, &[&account.name, &account.family])
            .await?;
        Ok(row.get(0))
    }

    pub(super) async fn get_account(&self, id: i32) -> Result<dto::Account, Box<dyn Error>> {
        let query = "SELECT id, name, family FROM accounts WHERE id = $1";
        let client = self.pool.get().await?;
        let row = client.query_one(query, &[&id]).await?;
        Ok(dto::Account {
            id: row.get(0),
            name: row.get(1),
            family: row.get(2),
        })
    }

    pub(super) async fn get_accounts(&self) -> Result<Vec<dto::Account>, Box<dyn Error>> {
        let query = "SELECT id, name, family FROM accounts";
        let client = self.pool.get().await?;
        let rows = client.query(query, &[]).await?;
        let accounts: Vec<dto::Account> = rows
            .iter()
            .map(|row| dto::Account {
                id: row.get(0),
                name: row.get(1),
                family: row.get(2),
            })
            .collect();
        Ok(accounts)
    }

    pub(super) async fn insert_entry(
        &self,
        entry: &dto::Entry,
    ) -> Result<i32, Box<dyn Error>> {
        let query = "INSERT INTO entries (description, amount, event_date, credit_id, debit_id) VALUES ($1, $2, $3, $4, $5) RETURNING id";
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                query,
                &[
                    &entry.description,
                    &entry.amount,
                    &entry.event_date,
                    &entry.credit_id,
                    &entry.debit_id,
                ],
            )
            .await?;
        Ok(row.get(0))
    }

    pub(super) async fn get_entry(&self, id: i32) -> Result<dto::Entry, Box<dyn Error>> {
        let query = "SELECT id, description, amount, event_date, credit_id, debit_id FROM entries WHERE id = $1";
        let client = self.pool.get().await?;
        let row = client.query_one(query, &[&id]).await?;
        Ok(dto::Entry {
            id: row.get(0),
            description: row.get(1),
            amount: row.get(2),
            event_date: row.get(3),
            credit_id: row.get(4),
            debit_id: row.get(5),
        })
    }
}