use crate::models::assets::{register_etf_asset, register_treasury_bond_asset};
use crate::schema::asset_prices;
use bigdecimal::{BigDecimal, Zero};
use chrono::NaiveDate;
use diesel::dsl::count_star;
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
) -> QueryResult<usize> {
    let prev_prices_count: i64 = asset_prices::table
        .select(count_star())
        .filter(asset_prices::asset_id.eq(asset_id))
        .get_result(conn)?;

    let new_prices_count = prices.len() as i64 - prev_prices_count;
    if new_prices_count <= 0 {
        return Ok(0);
    }

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

    Ok(new_prices_count as usize)
}

pub fn register_treasury_bond_prices(
    conn: &PgConnection,
    maturity_date: NaiveDate,
    prices: Vec<(NaiveDate, BigDecimal)>,
) -> QueryResult<usize> {
    conn.transaction(|| {
        let asset_id = register_treasury_bond_asset(conn, maturity_date)?;
        replace_asset_prices(conn, asset_id, prices)
    })
}

pub fn register_etf_prices(
    conn: &PgConnection,
    ticker: &str,
    prices: Vec<(NaiveDate, BigDecimal)>,
) -> QueryResult<usize> {
    conn.transaction(|| {
        let asset_id = register_etf_asset(conn, ticker)?;
        replace_asset_prices(conn, asset_id, prices)
    })
}

pub fn latest_prices(
    conn: &PgConnection,
    asset_ids: &[i32],
    until_date: NaiveDate,
) -> QueryResult<Vec<BigDecimal>> {
    let prices = asset_prices::table
        .select((asset_prices::asset_id, asset_prices::price))
        .distinct_on(asset_prices::asset_id)
        .filter(asset_prices::date.le(until_date))
        .filter(asset_prices::asset_id.eq_any(asset_ids))
        .order((asset_prices::asset_id, asset_prices::date.desc()))
        .load::<(i32, BigDecimal)>(conn)?;

    Ok(asset_ids
        .iter()
        .map(|asset_id| {
            prices
                .iter()
                .find_map(|(aid, price)| {
                    if aid == asset_id {
                        Some(price.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or(BigDecimal::zero())
        })
        .collect())
}
