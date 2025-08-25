use rocket::Rocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::NoTls;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::repository::RepositoryRealtimeUpdater;

#[macro_use]
extern crate rocket;

mod config;
mod model;
mod repository;
mod routes;
mod utils;

use crate::routes::ApiDoc;
use crate::routes::{
    create_account, create_entry, get_account, get_entries_from_date_to_date, get_entry,
};

#[launch]
async fn rocket() -> Rocket<rocket::Build> {
    // Load configuration
    let config_file = "config.toml".to_string();
    let database_config = read_config(config_file)
        .await
        .expect("Failed to read configuration");
    let pool = database_config
        .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
        .unwrap();
    let db_pool = pool.clone();

    // Repository
    let repository = Arc::new(Mutex::new(repository::Repository::new(db_pool).await));

    // Notifications from Postgres
    let realtime_update_repo = Arc::clone(&repository);
    let pg_config = database_config.get_pg_config().unwrap();
    tokio::spawn(async move {
        let realtime_updater = RepositoryRealtimeUpdater::new(realtime_update_repo.clone());
        realtime_updater.listen(pg_config, NoTls).await;
    });

    rocket::build()
        .mount(
            "/",
            routes![
                get_account,
                create_account,
                get_entry,
                create_entry,
                get_entries_from_date_to_date
            ],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .manage(repository)
}

async fn read_config(
    config_file: String,
) -> Result<deadpool_postgres::Config, Box<dyn std::error::Error>> {
    let config = config::load_config(config_file)?;

    let mut deadpool_config = deadpool_postgres::Config::new();
    deadpool_config.host = Some(config.database.url);
    deadpool_config.port = config.database.port.map(|p| {
        p.parse()
            .expect("Failed to parse port from config.toml. Ensure it's a valid u16.")
    });

    deadpool_config.user = Some(config.database.user);
    deadpool_config.password = Some(config.database.password);
    deadpool_config.dbname = Some(config.database.name);
    deadpool_config.manager = Some(deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });

    Ok(deadpool_config)
}
