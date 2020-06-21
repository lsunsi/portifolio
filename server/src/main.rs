#[macro_use]
extern crate diesel;

mod database;
mod models;
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database = database::init();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(database.clone())
            .configure(routes::config)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
