mod entities;
mod errors;
mod migrator;
mod models;
mod persistence;
mod progress;
mod routes;
mod validation_results;
mod validators;
mod ws;

#[macro_use]
extern crate rocket;

use migrator::Migrator;
use persistence::HistoryStore;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let client = reqwest::Client::new();

    let db_url = "sqlite://data/sqlite.db?mode=rwc";
    let conn = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&conn, None).await.expect("Migration failed");

    let history_store = HistoryStore::new(conn);

    rocket::build().manage(client).manage(history_store).mount(
        "/",
        routes![
            routes::validate,
            routes::validate_batch,
            routes::get_history,
            ws::validate_ws
        ],
    )
}
