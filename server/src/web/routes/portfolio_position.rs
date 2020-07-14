use crate::{
    database::Database,
    models::{position, AssetPosition, Assetable, PortfolioPosition},
    web::cookies::PortfolioId,
};
use actix_web::{web, HttpResponse};
use bigdecimal::ToPrimitive;
use chrono::{NaiveDate, Utc};
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "t", content = "c")]
enum ResponseAssetable {
    Treasury(NaiveDate),
    Etf(String),
}

#[derive(Serialize)]
struct ResponseAssetPosition {
    assetable: ResponseAssetable,
    amount: f32,
    price: f32,
    quantity: f32,
}

#[derive(Serialize)]
struct ResponsePortfolioPosition {
    assets: Vec<ResponseAssetPosition>,
    amount: f32,
}

impl From<Assetable> for ResponseAssetable {
    fn from(a: Assetable) -> ResponseAssetable {
        match a {
            Assetable::Treasury(t) => ResponseAssetable::Treasury(t.maturity_date),
            Assetable::Etf(etf) => ResponseAssetable::Etf(etf.ticker),
        }
    }
}

impl From<AssetPosition> for ResponseAssetPosition {
    fn from(ap: AssetPosition) -> ResponseAssetPosition {
        ResponseAssetPosition {
            assetable: ap.assetable.into(),
            amount: ap.amount.with_scale(2).to_f32().unwrap(),
            price: ap.price.with_scale(2).to_f32().unwrap(),
            quantity: ap.quantity.with_scale(2).to_f32().unwrap(),
        }
    }
}

impl From<PortfolioPosition> for ResponsePortfolioPosition {
    fn from(pp: PortfolioPosition) -> ResponsePortfolioPosition {
        ResponsePortfolioPosition {
            amount: pp.amount.with_scale(2).to_f32().unwrap(),
            assets: pp.assets.into_iter().map(Into::into).collect(),
        }
    }
}

#[actix_web::get("/portfolio-position")]
pub async fn get(db: web::Data<Database>, portfolio_id: PortfolioId) -> HttpResponse {
    let conn = db.get().unwrap();
    let today = Utc::now().date().naive_utc();

    let result = position(&conn, portfolio_id.0, today.into());

    match result {
        Err(_) => HttpResponse::InternalServerError().body("something bad is not right"),
        Ok(position) => HttpResponse::Ok().json::<ResponsePortfolioPosition>(position.into()),
    }
}
