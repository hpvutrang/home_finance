use rocket::{Rocket, http::Status, serde::json::Json};
use tokio_postgres::NoTls;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[macro_use]
extern crate rocket;

mod config;
mod model;
mod repository;
mod utils;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_account,
        create_account,
        get_entry,
        create_entry,
        get_entries_from_date_to_date,
    ),
    components(
        schemas(model::account::Account, model::entry::Entry, model::account::AccountFamily)
    ),
    tags(
        (name = "finance", description = "Finance management API")
    )
)]
struct ApiDoc;

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

    let repository = repository::Repository::new(db_pool).await;

    rocket::build()
        .mount(
            "/",
            routes![get_account, create_account, get_entry, create_entry, get_entries_from_date_to_date],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .manage(repository)
}

#[utoipa::path(
    get,
    path = "/account/{id}",
    responses(
        (status = 200, description = "Account found successfully", body = Account),
        (status = 404, description = "Account not found")
    ),
    params(
        ("id" = i32, Path, description = "Account id")
    )
)]
#[get("/account/<id>")]
async fn get_account(
    id: i32,
    repository: &rocket::State<repository::Repository>,
) -> Result<Json<model::account::Account>, Status> {
    match repository.get_account(id).await {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound),
    }
}

#[utoipa::path(
    post,
    path = "/account",
    request_body = Account,
    responses(
        (status = 201, description = "Account created successfully"),
    )
)]
#[post("/account", data = "<account>")]
async fn create_account(
    account: Json<model::account::Account>,
    repository: &rocket::State<repository::Repository>,
) -> Status {
    match repository.insert_account(&account.into_inner()).await {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[utoipa::path(
    get,
    path = "/entry/{id}",
    responses(
        (status = 200, description = "Entry found successfully", body = Entry),
        (status = 404, description = "Entry not found")
    ),
    params(
        ("id" = i32, Path, description = "Entry id")
    )
)]
#[get("/entry/<id>")]
async fn get_entry(
    id: i32,
    repository: &rocket::State<repository::Repository>,
) -> Result<Json<model::entry::Entry>, Status> {
    match repository.get_entry(id).await {
        Ok(entry) => Ok(Json(entry)),
        Err(_) => Err(Status::NotFound),
    }
}

#[utoipa::path(
    post,
    path = "/entry",
    request_body = Entry,
    responses(
        (status = 201, description = "Entry created successfully"),
    )
)]
#[post("/entry", data = "<entry>")]
async fn create_entry(
    entry: Json<model::entry::Entry>,
    repository: &rocket::State<repository::Repository>,
) -> Status {
    match repository.insert_entry(&entry.into_inner()).await {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[utoipa::path(
     get,
     path = "/entries",
     responses(
         (status = 200, description = "Entries retrieved successfully", body = [Entry]),
         (status = 500, description = "Internal server error")
     ),
     params(
         ("start_date" = Option<String>, Query, description = "Start date for filtering entries"),
         ("end_date" = Option<String>, Query, description = "End date for filtering entries")
     )
 )]
#[get("/entries?<start_date>&<end_date>")]
async fn get_entries_from_date_to_date(
    repository: &rocket::State<repository::Repository>,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<Json<Vec<model::entry::Entry>>, Status> {
    let mut filters = repository::filter::Filters::<repository::filter::EntryFields>::new();
    if let Some(start) = start_date {
        filters.and(
            &repository::filter::EntryFields::EventDate,
            repository::filter::Operator::GreaterThanOrEqual,
            start,
        );
    }
    if let Some(end) = end_date {
        filters.and(
            &repository::filter::EntryFields::EventDate,
            repository::filter::Operator::LessThanOrEqual,
            end,
        );
    }

    match repository.get_entries(&filters).await {
        Ok(entries) => Ok(Json(entries)),
        Err(e) => {
            eprintln!("Error retrieving entries: {}", e);
            Err(Status::InternalServerError)},
    }
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
