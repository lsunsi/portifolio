#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod database;
mod env;
mod models;
mod schema;
mod services;
mod web;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

embed_migrations!("./migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let env = env::init();

    let database = database::init(&env.database_url);
    embedded_migrations::run_with_output(&database.get().unwrap(), &mut std::io::stdout()).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&env.client_url)
                    .supports_credentials()
                    .finish(),
            )
            .wrap(Logger::default())
            .data(database.clone())
            .data(env.clone())
            .configure(web::routes::config)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
