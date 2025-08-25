use std::sync::Arc;
use tokio::sync::Mutex;

use rocket::{http::Status, serde::json::Json};

use utoipa::OpenApi;

use crate::{
    model,
    repository::{self, Repository},
};

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
pub struct ApiDoc;

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
pub async fn get_account(
    id: i32,
    repository: &rocket::State<Arc<Mutex<Repository>>>,
) -> Result<Json<model::account::Account>, Status> {
    // Clone the Arc to avoid holding the MutexGuard across await
    match repository.lock().await.get_account(id).await {
        Ok(entry) => Ok(Json(entry)),
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
pub async fn create_account(
    account: Json<model::account::Account>,
    repository: &rocket::State<Arc<Mutex<repository::Repository>>>,
) -> Status {
    match repository
        .lock()
        .await
        .insert_account(&account.into_inner())
        .await
    {
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
pub async fn get_entry(
    id: i32,
    repository: &rocket::State<Arc<Mutex<repository::Repository>>>,
) -> Result<Json<model::entry::Entry>, Status> {
    match repository.lock().await.get_entry(id).await {
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
pub async fn create_entry(
    entry: Json<model::entry::Entry>,
    repository: &rocket::State<Arc<Mutex<repository::Repository>>>,
) -> Status {
    match repository
        .lock()
        .await
        .insert_entry(&entry.into_inner())
        .await
    {
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
pub async fn get_entries_from_date_to_date(
    repository: &rocket::State<Arc<Mutex<repository::Repository>>>,
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

    match repository.lock().await.get_entries(&filters).await {
        Ok(entries) => Ok(Json(entries)),
        Err(e) => {
            eprintln!("Error retrieving entries: {}", e);
            Err(Status::InternalServerError)
        }
    }
}
