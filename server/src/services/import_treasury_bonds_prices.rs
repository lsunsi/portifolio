use crate::models::register_treasury_bond_prices;
use actix_web::client::Client;
use bigdecimal::BigDecimal;
use bytes::{buf::ext::BufExt, Bytes};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use diesel::PgConnection;
use itertools::Itertools;
use serde::Deserialize;
use std::str::FromStr;

pub enum ReadingError {
    Network(actix_web::client::SendRequestError),
    Status(actix_web::http::StatusCode),
    Payload(actix_web::client::PayloadError),
    Parsing(String),
}

pub enum WritingError {
    Writing(diesel::result::Error),
}

#[derive(Deserialize)]
struct Line {
    #[serde(rename = "Tipo Titulo")]
    kind: String,
    #[serde(rename = "Data Vencimento")]
    maturity: String,
    #[serde(rename = "Data Base")]
    date: String,
    #[serde(rename = "PU Base Manha")]
    price: String,
}

struct ParsedLine {
    key: &'static str,
    maturity: NaiveDate,
    date: NaiveDate,
    price: BigDecimal,
}

async fn fetch() -> Result<Bytes, ReadingError> {
    let request = Client::default().get(URL);
    let mut response = request.send().await.map_err(ReadingError::Network)?;

    let status = response.status();
    if status != 200 {
        return Err(ReadingError::Status(status));
    }

    response
        .body()
        .limit(16777216)
        .await
        .map_err(ReadingError::Payload)
}

fn parse(bytes: Bytes) -> Result<Vec<ParsedLine>, ReadingError> {
    let lines = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(bytes.reader())
        .into_deserialize();

    let mut treasury_prices = vec![];
    for line in lines {
        let line: Line =
            line.map_err(|e| ReadingError::Parsing(format!("Some line is bad: {}", e)))?;

        if line.price == "" || line.price == "0,00" {
            continue;
        }

        let key = match &line.kind[..] {
            "Tesouro Selic" => "LFT",
            "Tesouro IPCA+" => "NTN-B",
            "Tesouro IPCA+ com Juros Semestrais" => "NTN-B Principal",
            "Tesouro Prefixado" => "LTN",
            "Tesouro Prefixado com Juros Semestrais" => "NTN-F",
            "Tesouro IGPM+ com Juros Semestrais" => "NTN-C",
            other_key => {
                return Err(ReadingError::Parsing(format!(
                    "Some line type is bad: {}",
                    other_key
                )))
            }
        };

        let price = BigDecimal::from_str(&line.price.replace(",", "."))
            .map_err(|e| ReadingError::Parsing(format!("Some line price is bad: {}", e)))?;

        let date = NaiveDate::parse_from_str(&line.date, "%d/%m/%Y")
            .map_err(|e| ReadingError::Parsing(format!("Some line date is bad: {}", e)))?;

        let maturity = NaiveDate::parse_from_str(&line.maturity, "%d/%m/%Y")
            .map_err(|e| ReadingError::Parsing(format!("Some line maturity is bad: {}", e)))?;

        treasury_prices.push(ParsedLine {
            key,
            maturity,
            price,
            date,
        });
    }

    Ok(treasury_prices)
}

fn write(
    conn: &PgConnection,
    lines: Vec<ParsedLine>,
) -> Vec<((&'static str, NaiveDate), Result<usize, WritingError>)> {
    lines
        .into_iter()
        .sorted_by_key(|tp| (tp.key, tp.maturity))
        .group_by(|tp| (tp.key, tp.maturity))
        .into_iter()
        .map(|((key, maturity), treasury_prices)| {
            let prices = treasury_prices.map(|tp| (tp.date, tp.price)).collect();
            let res = register_treasury_bond_prices(conn, key, maturity, prices)
                .map_err(WritingError::Writing);
            ((key, maturity), res)
        })
        .collect()
}

pub async fn run(
    conn: &PgConnection,
) -> Result<Vec<((&'static str, NaiveDate), Result<usize, WritingError>)>, ReadingError> {
    fetch()
        .await
        .and_then(|bytes| parse(bytes))
        .map(|lines| write(conn, lines))
}

const URL: &'static str = "http://www.tesourotransparente.gov.br/ckan/dataset/df56aa42-484a-4a59-8184-7676580c81e3/resource/796d2059-14e9-44e3-80c9-2d9e30b405c1/download/PrecoTaxaTesouroDireto.csv";
