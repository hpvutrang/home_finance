use deadpool_postgres::Pool;

use crate::{
    model,
    repository::{self, cache::Repository as CacheRepository, dto::DtoModelNoRef},
};

mod cache;
mod dao;
mod dto;

pub struct Repository {
    dao: dao::Dao,
    account_repository: cache::AccountRepository,
}

impl Repository {
    pub async fn new(pool: Pool) -> Repository {
        let dao = dao::new(pool);
        let account_repository = initialize_account_repository(&dao).await;

        repository::Repository {
            dao,
            account_repository,
        }
    }

    pub async fn insert_account(
        &self,
        account: &model::account::Account,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let account_dto: dto::Account = dto::DtoModelNoRef::from_model(account);
        let res = self.dao.insert_account(&account_dto).await?;
        self.account_repository.add(res, account.clone()).await?;
        Ok(res)
    }

    pub async fn get_account(
        &self,
        id: i32,
    ) -> Result<model::account::Account, Box<dyn std::error::Error>> {
        if let Some(account) = self.account_repository.get(&id).await? {
            return Ok(account);
        }

        let account_dto = self.dao.get_account(id).await?;
        let account = dto::DtoModelNoRef::to_model(&account_dto);
        self.account_repository.add(id, account.clone()).await?;
        Ok(account)
    }

    pub async fn insert_entry(
        &self,
        entry: &model::entry::Entry,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut entry_dto: dto::Entry = dto::DtoModelNoRef::from_model(entry);

        entry_dto.credit_id = self
            .account_repository
            .get_id_by_name(entry.credit.name.as_str())
            .await?;
        entry_dto.debit_id = self
            .account_repository
            .get_id_by_name(entry.debit.name.as_str())
            .await?;

        let res = self.dao.insert_entry(&entry_dto).await?;

        Ok(res)
    }

    pub async fn get_entry(
        &self,
        id: i32,
    ) -> Result<model::entry::Entry, Box<dyn std::error::Error>> {
        let entry_dto = self.dao.get_entry(id).await?;
        let credit_account = self.get_account(entry_dto.credit_id).await?;
        let debit_account = self.get_account(entry_dto.debit_id).await?;

        let mut res = dto::DtoModelNoRef::to_model(&entry_dto);
        res.credit = credit_account;
        res.debit= debit_account;
        Ok(res)
    }
}

async fn initialize_account_repository(dao: &dao::Dao) -> cache::AccountRepository {
    let cache = cache::AccountRepository::new();
    let accounts = dao.get_accounts().await.expect("Failed to fetch accounts");
    for account in accounts {
        let _ = cache.add(account.id, account.to_model()).await;
    }

    cache
}