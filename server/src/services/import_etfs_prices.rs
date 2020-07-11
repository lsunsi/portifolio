use crate::models::register_etf_prices;
use actix_web::client::Client;
use bigdecimal::BigDecimal;
use bytes::Bytes;
use chrono::NaiveDate;
use diesel::PgConnection;
use scraper::{Html, Selector};
use std::str::FromStr;

pub enum Error {
    Network(actix_web::client::SendRequestError),
    Status(actix_web::http::StatusCode),
    Payload(actix_web::client::PayloadError),
    Writing(diesel::result::Error),
    Parsing(String),
}

async fn fetch() -> Result<Bytes, Error> {
    let mut response = Client::default()
        .post("https://br.investing.com/instruments/HistoricalDataAjax")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:77.0) Gecko/20100101 Firefox/77.0",
        )
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send_body("curr_id=39004&smlID=2514218&header=BOVA11+-+Dados+Hist%C3%B3ricos&st_date=28%2F05%2F2000&end_date=28%2F06%2F2020&interval_sec=Daily&sort_col=date&sort_ord=DESC&action=historical_data")
        .await.map_err(Error::Network)?;

    let status = response.status();
    if status != 200 {
        return Err(Error::Status(status));
    }

    response
        .body()
        .limit(PAYLOAD_LIMIT)
        .await
        .map_err(Error::Payload)
}

fn parse(bytes: Bytes) -> Result<Vec<(NaiveDate, BigDecimal)>, Error> {
    let string = std::str::from_utf8(&bytes)
        .map_err(|_| Error::Parsing("Body is not a utf8 string".into()))?;

    let fragment = Html::parse_fragment(string);

    let date_header_selector =
        Selector::parse("table#curr_table>thead>tr>th:nth-child(1)[data-col-name='date']")
            .map_err(|_| Error::Parsing("Date header selector parsing failed".into()))?;

    let price_header_selector =
        Selector::parse("table#curr_table>thead>tr>th:nth-child(2)[data-col-name='price']")
            .map_err(|_| Error::Parsing("Price header selector parsing failed".into()))?;

    if fragment.select(&date_header_selector).count() != 1 {
        return Err(Error::Parsing("Date header checking failed".into()));
    }

    if fragment.select(&price_header_selector).into_iter().count() != 1 {
        return Err(Error::Parsing("Price header checking failed".into()));
    }

    let rows_selector = Selector::parse("table#curr_table>tbody>tr")
        .map_err(|_| Error::Parsing("Rows selector parsing failed".into()))?;

    let date_selector = Selector::parse("td:nth-child(1)")
        .map_err(|_| Error::Parsing("Date selector failed".into()))?;

    let price_selector = Selector::parse("td:nth-child(2)")
        .map_err(|_| Error::Parsing("Price selector failed".into()))?;

    let mut prices = vec![];
    for row in fragment.select(&rows_selector) {
        let date = row
            .select(&date_selector)
            .next()
            .and_then(|el| el.text().next())
            .and_then(|raw| NaiveDate::parse_from_str(raw, "%d.%m.%Y").ok())
            .ok_or(Error::Parsing("Date parsing failed on some row".into()))?;

        let price = row
            .select(&price_selector)
            .next()
            .and_then(|el| el.text().next())
            .and_then(|raw| BigDecimal::from_str(&String::from(raw).replace(",", ".")).ok())
            .ok_or(Error::Parsing("Price parsing failed on some row".into()))?;

        prices.push((date, price));
    }

    Ok(prices)
}

fn write(
    conn: &PgConnection,
    ticker: &str,
    lines: Vec<(NaiveDate, BigDecimal)>,
) -> Result<(), Error> {
    register_etf_prices(conn, ticker, lines).map_err(Error::Writing)
}

pub async fn run(conn: &PgConnection) -> Vec<(&'static str, Result<(), Error>)> {
    let ticker = "BOVA11";

    let res = fetch()
        .await
        .and_then(parse)
        .and_then(|lines| write(conn, ticker, lines));

    vec![(ticker, res)]
}

const PAYLOAD_LIMIT: usize = 16777216;
