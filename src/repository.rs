use std::sync::Arc;

use deadpool_postgres::Pool;
use tokio::sync::Mutex;
use tokio_postgres::{
    Config, Socket,
    tls::{MakeTlsConnect, TlsConnect},
};
use tracing::{Level, instrument};

mod cache;
mod dao;
mod db_listener;
mod dto;

pub mod filter;

use crate::{
    model,
    repository::{
        self,
        cache::Repository as CacheRepository,
        db_listener::{DatabaseListener, NotificationHandler},
        dto::DtoModelNoRef,
    },
};

pub struct Repository {
    dao: dao::Dao,
    account_repository: cache::AccountRepository,
}

impl Repository {
    #[instrument(name = "Repository initialization", skip(pool))]
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
        res.debit = debit_account;
        Ok(res)
    }

    pub async fn get_entries(
        &self,
        filter: &filter::Filters<filter::EntryFields>,
    ) -> Result<Vec<model::entry::Entry>, Box<dyn std::error::Error>> {
        let entries_dto = self.dao.get_entries(filter).await?;

        let mut entries = Vec::new();
        for entry_dto in entries_dto {
            let credit_account = self.get_account(entry_dto.credit_id).await?;
            let debit_account = self.get_account(entry_dto.debit_id).await?;
            let mut entry = dto::DtoModelNoRef::to_model(&entry_dto);
            entry.credit = credit_account;
            entry.debit = debit_account;
            entries.push(entry);
        }

        Ok(entries)
    }
}

pub struct RepositoryRealtimeUpdater {
    shared_repository: Arc<Mutex<Repository>>,
}

impl RepositoryRealtimeUpdater {
    pub fn new(repository: Arc<Mutex<Repository>>) -> Self {
        Self {
            shared_repository: repository,
        }
    }

    pub async fn listen<T>(&self, pg_config: Config, tls: T)
    where
        T: MakeTlsConnect<Socket> + Clone + Sync + Send + 'static,
        T::Stream: Sync + Send,
        T::TlsConnect: Sync + Send,
        <T::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let updater = AccountRepositoryRealtimeUpdater::new(self.shared_repository.clone());
        let connect = DatabaseListener::new(pg_config, tls);
        connect.attach(updater, "todo").await;
    }
}

pub struct AccountRepositoryRealtimeUpdater {
    _shared_repository: Arc<Mutex<Repository>>,
}

impl AccountRepositoryRealtimeUpdater {
    fn new(shared_repository: Arc<Mutex<Repository>>) -> Self {
        Self {
            _shared_repository: shared_repository,
        }
    }
}

impl NotificationHandler for AccountRepositoryRealtimeUpdater {
    fn on_notification_received(&self, _channel: &str, _message: &str) {
        todo!("TO BE IMPLEMENTED")
    }
}

#[instrument(name = "Account repository initialization", level = Level::DEBUG, skip(dao))]
async fn initialize_account_repository(dao: &dao::Dao) -> cache::AccountRepository {
    let cache = cache::AccountRepository::new();
    let accounts = dao.get_accounts().await.expect("Failed to fetch accounts");
    for account in accounts {
        let _ = cache.add(account.id, account.to_model()).await;
    }

    cache
}
