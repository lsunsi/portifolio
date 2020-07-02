use crate::models::assets::{register_etf_asset, register_treasury_asset};
use crate::schema::asset_prices;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "asset_prices"]
struct NewAssetPrice {
    asset_id: i32,
    price: BigDecimal,
    date: NaiveDate,
}
fn replace_asset_prices(
    conn: &PgConnection,
    asset_id: i32,
    prices: Vec<(NaiveDate, BigDecimal)>,
) -> QueryResult<()> {
    diesel::delete(asset_prices::table.filter(asset_prices::asset_id.eq(asset_id)))
        .execute(conn)?;

    let insertable_prices: Vec<_> = prices
        .into_iter()
        .map(|(date, price)| NewAssetPrice {
            asset_id,
            price,
            date,
        })
        .collect();

    diesel::insert_into(asset_prices::table)
        .values(insertable_prices)
        .execute(conn)?;

    Ok(())
}

pub fn register_treasury_prices(
    conn: &PgConnection,
    maturity_date: NaiveDate,
    prices: Vec<(NaiveDate, BigDecimal)>,
) -> QueryResult<()> {
    conn.transaction(|| {
        let asset_id = register_treasury_asset(conn, maturity_date)?;
        replace_asset_prices(conn, asset_id, prices)?;
        Ok(())
    })
}

pub fn register_etf_prices(
    conn: &PgConnection,
    ticker: String,
    prices: Vec<(NaiveDate, BigDecimal)>,
) -> QueryResult<()> {
    conn.transaction(|| {
        let asset_id = register_etf_asset(conn, ticker)?;
        replace_asset_prices(conn, asset_id, prices)?;
        Ok(())
    })
}
