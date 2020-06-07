use actix_web::web::Data;
use actix_web::{get, App, HttpServer};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

type Database = Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(_: Data<Database>) -> &'static str {
    "oie"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = Pool::builder().build(manager).unwrap();

    HttpServer::new(move || App::new().data(pool.clone()).service(index))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
