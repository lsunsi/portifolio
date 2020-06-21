use crate::database::Database;
use crate::models::register_treasury_prices;
use actix_web::{client::Client, web::Data, HttpResponse};
use bigdecimal::BigDecimal;
use bytes::{buf::ext::BufExt, Bytes};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use diesel::PgConnection;
use itertools::Itertools;
use serde::Deserialize;
use std::str::FromStr;

const URL: &'static str = "http://www.tesourotransparente.gov.br/ckan/dataset/df56aa42-484a-4a59-8184-7676580c81e3/resource/796d2059-14e9-44e3-80c9-2d9e30b405c1/download/PrecoTaxaTesouroDireto.csv";

#[derive(Deserialize)]
struct Line {
    #[serde(rename = "Tipo Titulo")]
    kind: String,
    #[serde(rename = "Data Vencimento")]
    maturity_date: String,
    #[serde(rename = "Data Base")]
    date: String,
    #[serde(rename = "PU Base Manha")]
    price: String,
}

struct TreasuryPrice {
    maturity_date: NaiveDate,
    date: NaiveDate,
    price: BigDecimal,
}

async fn fetch() -> Result<Bytes, &'static str> {
    let request = Client::default().get(URL);
    let mut response = request.send().await.or(Err("response is bad"))?;
    response.body().limit(16777216).await.or(Err("body is bad"))
}

fn parse(bytes: Bytes) -> Result<Vec<TreasuryPrice>, &'static str> {
    let lines = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(bytes.reader())
        .into_deserialize();

    let mut treasury_prices = vec![];
    for line in lines {
        if let Err(_) = line {
            return Err("some line is bad");
        };

        let line: Line = line.unwrap();

        if line.kind != "Tesouro Selic" || line.price == "" {
            continue;
        }

        let price = match BigDecimal::from_str(&line.price.replace(",", ".")) {
            Err(_) => return Err("some line price is bad"),
            Ok(price) => price,
        };

        let date = match NaiveDate::parse_from_str(&line.date, "%d/%m/%Y") {
            Err(_) => return Err("some line date is bad"),
            Ok(date) => date,
        };

        let maturity_date = match NaiveDate::parse_from_str(&line.maturity_date, "%d/%m/%Y") {
            Err(_) => return Err("some line maturity date is bad"),
            Ok(maturity_date) => maturity_date,
        };

        treasury_prices.push(TreasuryPrice {
            maturity_date,
            price,
            date,
        });
    }

    Ok(treasury_prices)
}

fn write(conn: &PgConnection, lines: Vec<TreasuryPrice>) -> Result<(), &'static str> {
    for (maturity_date, treasury_prices) in &lines
        .into_iter()
        .sorted_by_key(|tp| tp.maturity_date)
        .group_by(|tp| tp.maturity_date)
    {
        let prices = treasury_prices.map(|tp| (tp.date, tp.price)).collect();
        println!("{}", maturity_date);

        if let Err(_) = register_treasury_prices(conn, maturity_date, prices) {
            return Err("writing failed");
        };
    }

    Ok(())
}

#[actix_web::post("/import-treasury-prices")]
pub async fn post(db: Data<Database>) -> HttpResponse {
    let conn = db.get().unwrap();

    let result = fetch()
        .await
        .and_then(|bytes| parse(bytes))
        .and_then(|lines| write(&conn, lines));

    match result {
        Err(reason) => HttpResponse::InternalServerError().body(reason),
        Ok(()) => HttpResponse::Ok().finish(),
    }
}
