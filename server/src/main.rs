#[macro_use]
extern crate diesel;

mod database;
mod models;
mod schema;
mod services;
mod web;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database = database::init();
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .supports_credentials()
                    .allowed_origin("http://localhost:3000")
                    .finish(),
            )
            .wrap(Logger::default())
            .data(database.clone())
            .configure(web::routes::config)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
