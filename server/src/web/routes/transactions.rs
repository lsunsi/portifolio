use crate::{
    database::Database,
    models::Assetable,
    services::get_transactions::{self, Transaction},
    web::cookies::PortfolioId,
};
use actix_web::{web, HttpResponse};
use bigdecimal::ToPrimitive;
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
enum ResponseAssetable {
    Treasury(NaiveDate),
    Etf(String),
}

#[derive(Serialize)]
struct ResponseTransaction {
    assetable: ResponseAssetable,
    date: NaiveDate,
    price: f64,
    quantity: f64,
    amount: f64,
}

impl From<Assetable> for ResponseAssetable {
    fn from(a: Assetable) -> ResponseAssetable {
        match a {
            Assetable::Treasury(t) => ResponseAssetable::Treasury(t.maturity_date),
            Assetable::Etf(etf) => ResponseAssetable::Etf(etf.ticker),
        }
    }
}

impl From<Transaction> for ResponseTransaction {
    fn from(t: Transaction) -> ResponseTransaction {
        ResponseTransaction {
            price: t.price.with_scale(8).to_f64().unwrap(),
            quantity: t.quantity.with_scale(8).to_f64().unwrap(),
            amount: t.amount.with_scale(2).to_f64().unwrap(),
            assetable: t.assetable.into(),
            date: t.date,
        }
    }
}

#[actix_web::get("/transactions")]
pub async fn get(db: web::Data<Database>, portfolio_id: PortfolioId) -> HttpResponse {
    let conn = db.get().unwrap();

    let result = get_transactions::run(&conn, portfolio_id.0);

    match result {
        Err(_) => HttpResponse::InternalServerError().body("something bad is not right"),
        Ok(ts) => HttpResponse::Ok()
            .json::<Vec<ResponseTransaction>>(ts.into_iter().map(Into::into).collect()),
    }
}
