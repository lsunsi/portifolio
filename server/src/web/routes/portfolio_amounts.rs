use crate::{
    database::Database,
    services::get_portfolio_amounts::{self, PortfolioAmount},
    web::cookies::PortfolioId,
};
use actix_web::{web, HttpResponse};
use bigdecimal::ToPrimitive;
use chrono::{NaiveDate, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct ResponsePortfolioAmount((NaiveDate, f32, f32));

impl From<PortfolioAmount> for ResponsePortfolioAmount {
    fn from(pa: PortfolioAmount) -> ResponsePortfolioAmount {
        ResponsePortfolioAmount((
            pa.date,
            pa.invested.with_scale(2).to_f32().unwrap(),
            pa.gross_total.with_scale(2).to_f32().unwrap(),
        ))
    }
}

#[actix_web::get("/portfolio-amounts")]
pub async fn get(db: web::Data<Database>, portfolio_id: PortfolioId) -> HttpResponse {
    let conn = db.get().unwrap();
    let today = Utc::now().date().naive_utc();

    let result = get_portfolio_amounts::run(&conn, portfolio_id.0, today);

    match result {
        Err(_) => HttpResponse::InternalServerError().body("something bad is not right"),
        Ok(amounts) => HttpResponse::Ok()
            .json::<Vec<ResponsePortfolioAmount>>(amounts.into_iter().map(Into::into).collect()),
    }
}
