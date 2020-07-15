use crate::database::Database;
use crate::models::register_trades;
use crate::web::cookies::portfolio_id_cookie;
use actix_web::{web, HttpResponse, Result};
use bigdecimal::BigDecimal;
use bytes::{buf::ext::BufExt, Bytes};
use chrono::NaiveDate;
use csv::Reader;
use diesel::PgConnection;
use futures::StreamExt;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
struct Line {
    #[serde(rename = "Data")]
    date: String,
    #[serde(rename = "Preço")]
    price: String,
    #[serde(rename = "Quantidade")]
    quantity: String,
    #[serde(rename = "TipoAtivo")]
    asset_kind: String,
    #[serde(rename = "DadoAtivo")]
    asset_prop: String,
}

type EtfTrade = (String, NaiveDate, BigDecimal, BigDecimal);
type TreasuryTrade = (NaiveDate, NaiveDate, BigDecimal, BigDecimal);
type Trades = (Vec<EtfTrade>, Vec<TreasuryTrade>);

fn parse(bytes: Bytes) -> Result<(Vec<EtfTrade>, Vec<TreasuryTrade>), &'static str> {
    let lines = Reader::from_reader(bytes.reader()).into_deserialize();

    let mut etf_trades = vec![];
    let mut treasury_trades = vec![];

    for line in lines {
        let line: Line = match line {
            Err(_) => return Err("some line is bad"),
            Ok(line) => line,
        };

        let date = match NaiveDate::parse_from_str(&line.date, "%Y-%m-%d") {
            Err(_) => return Err("some line's date is bad"),
            Ok(date) => date,
        };

        let price = match BigDecimal::from_str(&line.price) {
            Err(_) => return Err("some line's price is bad"),
            Ok(date) => date,
        };

        let quantity = match BigDecimal::from_str(&line.quantity) {
            Err(_) => return Err("some line's quantity is bad"),
            Ok(date) => date,
        };

        match line.asset_kind.as_str() {
            "LFT" => {
                let maturity_date = match NaiveDate::parse_from_str(&line.asset_prop, "%Y-%m-%d") {
                    Err(_) => return Err("some line's treasury asset prop is bad"),
                    Ok(date) => date,
                };

                treasury_trades.push((maturity_date, date, price, quantity));
            }
            "ETF" => {
                let ticker = match line.asset_prop.as_str() {
                    "BOVA11" | "SMAL11" | "IVVB11" => line.asset_prop,
                    _ => return Err("some line's etf asset prop is bad"),
                };

                etf_trades.push((ticker, date, price, quantity));
            }
            _ => return Err("some line's asset kind is bad"),
        };
    }

    if etf_trades.len() + treasury_trades.len() == 0 {
        return Err("there were no trades");
    }

    Ok((etf_trades, treasury_trades))
}

fn write(conn: &PgConnection, (etf_trades, treasury_trades): Trades) -> Result<i32, &'static str> {
    register_trades(conn, etf_trades, treasury_trades).or(Err("writing trades failed"))
}

#[actix_web::post("/import-trades")]
pub async fn post(db: web::Data<Database>, mut data: web::Payload) -> HttpResponse {
    let conn = db.get().unwrap();

    let mut source = web::BytesMut::new();
    while let Some(item) = data.next().await {
        source.extend_from_slice(&item.unwrap());
    }

    match parse(source.into()).and_then(|trades| write(&conn, trades)) {
        Err(reason) => HttpResponse::InternalServerError().body(reason),
        Ok(id) => HttpResponse::Ok().cookie(portfolio_id_cookie(id)).finish(),
    }
}
