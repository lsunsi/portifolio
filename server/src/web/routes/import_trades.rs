use crate::database::Database;
use crate::services::import_trades::{run, Error};
use crate::web::cookies::portfolio_id_cookie;
use actix_web::{web, HttpResponse};
use futures::StreamExt;

#[actix_web::post("/import-trades")]
pub async fn post(db: web::Data<Database>, mut data: web::Payload) -> HttpResponse {
    let conn = db.get().unwrap();

    let mut csv = web::BytesMut::new();
    while let Some(item) = data.next().await {
        csv.extend_from_slice(&item.unwrap());
    }

    match run(&conn, csv.freeze()) {
        Err(Error::Parsing(e)) => HttpResponse::BadRequest().body(format!("ParsingError: {}", e)),
        Err(Error::Writing(e)) => HttpResponse::BadRequest().body(format!("WritingError: {}", e)),
        Ok(id) => HttpResponse::Created()
            .cookie(portfolio_id_cookie(id))
            .finish(),
    }
}
