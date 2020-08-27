use std::env::var;

#[derive(Clone)]
pub struct Env {
    pub database_url: String,
    pub client_url: String,
    pub domain: String,
}

pub fn init() -> Env {
    Env {
        database_url: var("DATABASE_URL").expect("DATABASE_URL"),
        client_url: var("CLIENT_URL").expect("CLIENT_URL"),
        domain: var("DOMAIN").expect("DOMAIN"),
    }
}
