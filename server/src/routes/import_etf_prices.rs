use crate::database::Database;
use crate::models::register_etf_prices;
use actix_web::{client::Client, web::Data, HttpResponse};
use bigdecimal::BigDecimal;
use bytes::Bytes;
use chrono::NaiveDate;
use diesel::PgConnection;
use scraper::{Html, Selector};
use std::str::FromStr;

async fn fetch() -> Result<Bytes, &'static str> {
    let request = Client::default()
        .post("https://br.investing.com/instruments/HistoricalDataAjax")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:77.0) Gecko/20100101 Firefox/77.0",
        )
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send_body("curr_id=39004&smlID=2514218&header=BOVA11+-+Dados+Hist%C3%B3ricos&st_date=28%2F05%2F2000&end_date=28%2F06%2F2020&interval_sec=Daily&sort_col=date&sort_ord=DESC&action=historical_data");

    let mut response = request.await.or(Err("response is bad"))?;
    if response.status() != 200 {
        return Err("status is bad");
    }

    response.body().limit(16777216).await.or(Err("body is bad"))
}

fn parse(bytes: Bytes) -> Result<Vec<(NaiveDate, BigDecimal)>, &'static str> {
    let string = match std::str::from_utf8(&bytes) {
        Err(_) => return Err("body is not a string"),
        Ok(s) => s,
    };

    let fragment = Html::parse_fragment(string);

    let date_header_selector =
        match Selector::parse("table#curr_table>thead>tr>th:nth-child(1)[data-col-name='date']") {
            Err(_) => return Err("bad date header selector"),
            Ok(selector) => selector,
        };

    let price_header_selector =
        match Selector::parse("table#curr_table>thead>tr>th:nth-child(2)[data-col-name='price']") {
            Err(_) => return Err("bad price header selector"),
            Ok(selector) => selector,
        };

    if fragment.select(&date_header_selector).into_iter().count() != 1 {
        return Err("date header is bad");
    }

    if fragment.select(&price_header_selector).into_iter().count() != 1 {
        return Err("price header is bad");
    }

    let rows_selector = match Selector::parse("table#curr_table>tbody>tr") {
        Err(_) => return Err("bad rows selector"),
        Ok(selector) => selector,
    };
    let date_selector = match Selector::parse("td:nth-child(1)") {
        Err(_) => return Err("bad date selector"),
        Ok(selector) => selector,
    };
    let price_selector = match Selector::parse("td:nth-child(2)") {
        Err(_) => return Err("bad price selector"),
        Ok(selector) => selector,
    };

    let mut prices = vec![];
    for row in fragment.select(&rows_selector) {
        let date = row
            .select(&date_selector)
            .into_iter()
            .next()
            .and_then(|el| el.text().next())
            .and_then(|raw| NaiveDate::parse_from_str(raw, "%d.%m.%Y").ok());

        let price = row
            .select(&price_selector)
            .into_iter()
            .next()
            .and_then(|el| el.text().next())
            .and_then(|raw| BigDecimal::from_str(&String::from(raw).replace(",", ".")).ok());

        match (date, price) {
            (Some(date), Some(price)) => prices.push((date, price)),
            _ => return Err("some line is bad"),
        }
    }

    Ok(prices)
}

fn write(conn: &PgConnection, lines: Vec<(NaiveDate, BigDecimal)>) -> Result<(), &'static str> {
    register_etf_prices(conn, "BOVA11".into(), lines).or(Err("writing failed"))
}

#[actix_web::post("/import-etf-prices")]
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
