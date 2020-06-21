use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type Database = Pool<ConnectionManager<PgConnection>>;

pub fn init() -> Database {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    Pool::builder().build(manager).unwrap()
}
