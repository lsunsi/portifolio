use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type Database = Pool<ConnectionManager<PgConnection>>;

pub fn init(connspec: &str) -> Database {
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    Pool::builder().build(manager).unwrap()
}
