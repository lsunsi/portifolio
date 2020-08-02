use crate::database::Database;
use crate::services::import_treasury_bonds_prices::{run, ReadingError};
use actix_web::{web::Data, HttpResponse};

#[actix_web::post("/import-treasury-bonds-prices")]
pub async fn post(db: Data<Database>) -> HttpResponse {
    let conn = db.get().unwrap();

    match run(&conn).await {
        Err(ReadingError::Network(e)) => HttpResponse::Ok().body(format!("Network Error: {:?}", e)),
        Err(ReadingError::Parsing(e)) => HttpResponse::Ok().body(format!("Bad Parsing: {:?}", e)),
        Err(ReadingError::Status(e)) => HttpResponse::Ok().body(format!("Bad Status: {:?}", e)),
        Err(ReadingError::Payload(_)) => HttpResponse::Ok().body("Bad Payload"),
        Ok(results) => {
            let bs: Vec<_> = results
                .into_iter()
                .filter(|(_, result)| {
                    result
                        .as_ref()
                        .ok()
                        .filter(|new_prices| *new_prices > &0)
                        .is_some()
                })
                .map(|((key, maturity), result)| {
                    format!("{}({}) : {:?}", key, maturity, result.map_err(|_| "Error"))
                })
                .collect();

            HttpResponse::Ok().json(bs)
        }
    }
}
