#[macro_use]
extern crate rocket;

use controllers::{Response, SuccessResponse};
use fairings::cors::{options, Cors};
use migrator::Migrator;
use rocket::http::Status;
use routes::{auth_routes, author_routes, book_routes};
use sea_orm_migration::prelude::*;
use config::AppConfig;

mod auth;
mod controllers;
mod db;
mod entities;
mod fairings;
mod migrator;
mod config;
mod routes;

#[get("/")]
fn index() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello, World".to_string())))
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let config = AppConfig::default();

    let db = db::connect(&config).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    rocket::build()
        .attach(Cors)
        .manage(db)
        .manage(config)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount("/auth", auth_routes())
        .mount("/authors", author_routes())
        .mount("/books", book_routes())
}
