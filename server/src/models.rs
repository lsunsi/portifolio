use crate::schema::{asset_prices, assets, treasuries};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[allow(dead_code)]
#[derive(Queryable)]
struct Asset {
    id: i32,
    kind: String,
}

#[allow(dead_code)]
#[derive(Queryable)]
struct Treasury {
    id: i32,
    kind: String,
    maturity_date: NaiveDate,
}

#[derive(Insertable)]
#[table_name = "assets"]
struct NewAsset {
    kind: &'static str,
}

#[derive(Insertable)]
#[table_name = "treasuries"]
struct NewTreasury {
    id: i32,
    maturity_date: NaiveDate,
}

#[derive(Insertable)]
#[table_name = "asset_prices"]
struct NewAssetPrice {
    asset_id: i32,
    price: BigDecimal,
    date: NaiveDate,
}

fn register_treasury_asset(conn: &PgConnection, maturity_date: NaiveDate) -> QueryResult<i32> {
    let treasury = treasuries::table
        .filter(treasuries::maturity_date.eq(maturity_date))
        .first::<Treasury>(conn)
        .optional()?;

    if let Some(treasury) = treasury {
        return Ok(treasury.id);
    }

    let asset = diesel::insert_into(assets::table)
        .values(&NewAsset { kind: "treasury" })
        .get_result::<Asset>(conn)?;

    let treasury = diesel::insert_into(treasuries::table)
        .values(&NewTreasury {
            id: asset.id,
            maturity_date,
        })
        .get_result::<Treasury>(conn)?;

    Ok(treasury.id)
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
