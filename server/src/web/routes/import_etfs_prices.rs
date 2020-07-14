use crate::database::Database;
use crate::services::import_etfs_prices;
use actix_web::{web::Data, HttpResponse};

#[actix_web::post("/import-etfs-prices")]
pub async fn post(db: Data<Database>) -> HttpResponse {
    let conn = db.get().unwrap();

    let results = import_etfs_prices::run(&conn)
        .await
        .into_iter()
        .map(|(ticker, result)| {
            (
                ticker,
                match result {
                    Ok(new_prices_count) => Ok(new_prices_count),
                    Err(import_etfs_prices::Error::Network(_)) => Err("Network"),
                    Err(import_etfs_prices::Error::Status(_)) => Err("Status"),
                    Err(import_etfs_prices::Error::Payload(_)) => Err("Payload"),
                    Err(import_etfs_prices::Error::Parsing(_)) => Err("Parsing"),
                    Err(import_etfs_prices::Error::Writing(_)) => Err("Writing"),
                },
            )
        })
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(results)
}
