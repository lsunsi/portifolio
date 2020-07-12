use crate::models::register_etf_prices;
use actix_web::client::Client;
use bigdecimal::BigDecimal;
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

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
struct FormParams {
    curr_id: &'static str,
    smlID: &'static str,
    header: &'static str,
    st_date: &'static str,
    end_date: &'static str,
    interval_sec: &'static str,
    sort_col: &'static str,
    sort_ord: &'static str,
    action: &'static str,
}

async fn fetch(form_params: &FormParams) -> Result<bytes::Bytes, Error> {
    let mut response = Client::default()
        .post("https://br.investing.com/instruments/HistoricalDataAjax")
        .header("Accept", "text/plain, */*; q=0.01")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Origin", "https://www.investing.com")
        .header("DNT", "1")
        .header("Connection", "keep-alive")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0",
        )
        .send_form(form_params)
        .await
        .map_err(Error::Network)?;

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

fn parse(bytes: bytes::Bytes) -> Result<Vec<(NaiveDate, BigDecimal)>, Error> {
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
) -> Result<usize, Error> {
    register_etf_prices(conn, ticker, lines).map_err(Error::Writing)
}

pub async fn run(conn: &PgConnection) -> Vec<(&'static str, Result<usize, Error>)> {
    let tickers_params = [
        ("BOVA11", BOVA11_FORM_PARAMS),
        ("SMAL11", SMAL11_FORM_PARAMS),
        ("IVVB11", IVVB11_FORM_PARAMS),
    ];

    let mut results = vec![];

    for (ticker, params) in tickers_params.iter() {
        let result = fetch(params)
            .await
            .and_then(parse)
            .and_then(|lines| write(conn, ticker, lines));

        results.push((*ticker, result));
    }

    results
}

const PAYLOAD_LIMIT: usize = 16777216;

const BOVA11_FORM_PARAMS: FormParams = FormParams {
    curr_id: "39004",
    smlID: "2514218",
    header: "BOVA11+Historical+Data",
    st_date: "01/01/2000",
    end_date: "01/01/2022",
    interval_sec: "Daily",
    sort_col: "date",
    sort_ord: "DESC",
    action: "historical_data",
};

const SMAL11_FORM_PARAMS: FormParams = FormParams {
    curr_id: "39013",
    smlID: "2514317",
    header: "SMAL11+Historical+Data",
    st_date: "01/01/2000",
    end_date: "01/01/2022",
    interval_sec: "Daily",
    sort_col: "date",
    sort_ord: "DESC",
    action: "historical_data",
};

const IVVB11_FORM_PARAMS: FormParams = FormParams {
    curr_id: "956435",
    smlID: "2585374",
    header: "IVVB11+Historical+Data",
    st_date: "01/01/2000",
    end_date: "01/01/2022",
    interval_sec: "Daily",
    sort_col: "date",
    sort_ord: "DESC",
    action: "historical_data",
};
