use crate::models::{register_trades, EtfTrade, TreasuryBondTrade};
use bigdecimal::BigDecimal;
use bytes::{buf::ext::BufExt, Bytes};
use chrono::NaiveDate;
use csv::Reader;
use diesel::PgConnection;
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
    #[serde(rename = "DadoAtivo1")]
    asset_prop_1: String,
    #[serde(rename = "DadoAtivo2")]
    asset_prop_2: String,
}

pub enum Error {
    Writing(diesel::result::Error),
    Parsing(String),
}

fn parse(bytes: Bytes) -> Result<(Vec<EtfTrade>, Vec<TreasuryBondTrade>), String> {
    let lines = Reader::from_reader(bytes.reader()).into_deserialize();

    let mut etf_trades = vec![];
    let mut treasury_bond_trades = vec![];

    for line in lines {
        let line: Line = line.map_err(|e| format!("Some line is bad: {}", e))?;

        let date = NaiveDate::parse_from_str(&line.date, "%d/%m/%Y")
            .map_err(|e| format!("Some line's date is bad: {}", e))?;

        let price = BigDecimal::from_str(&line.price)
            .map_err(|e| format!("Some line's price is bad: {}", e))?;

        let quantity = BigDecimal::from_str(&line.quantity)
            .map_err(|e| format!("Some line's quantity is bad: {}", e))?;

        match line.asset_kind.as_str() {
            "Tesouro" => {
                let maturity = NaiveDate::parse_from_str(&line.asset_prop_2, "%d/%m/%Y")
                    .map_err(|e| format!("Some line's treasury maturity is bad: {}", e))?;

                treasury_bond_trades.push(TreasuryBondTrade {
                    key: line.asset_prop_1,
                    maturity,
                    quantity,
                    price,
                    date,
                });
            }
            "ETF" => {
                etf_trades.push(EtfTrade {
                    ticker: line.asset_prop_1,
                    quantity,
                    price,
                    date,
                });
            }
            kind => return Err(format!("Some line's asset kind is bad: {}", kind)),
        };
    }

    if etf_trades.len() + treasury_bond_trades.len() == 0 {
        return Err(String::from("There were no trades to import"));
    }

    Ok((etf_trades, treasury_bond_trades))
}

pub fn run(conn: &PgConnection, csv: Bytes) -> Result<i32, Error> {
    parse(csv)
        .map_err(Error::Parsing)
        .and_then(|(etf_trades, treasury_bond_trades)| {
            register_trades(conn, &etf_trades, &treasury_bond_trades).map_err(Error::Writing)
        })
}
