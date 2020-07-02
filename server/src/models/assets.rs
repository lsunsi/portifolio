use crate::schema::{assets, etfs, treasuries};
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

#[allow(dead_code)]
#[derive(Queryable)]
struct Etf {
    id: i32,
    kind: String,
    ticker: String,
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
#[table_name = "etfs"]
struct NewEtf {
    id: i32,
    ticker: String,
}

pub fn register_treasury_asset(conn: &PgConnection, maturity_date: NaiveDate) -> QueryResult<i32> {
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

pub fn register_etf_asset(conn: &PgConnection, ticker: String) -> QueryResult<i32> {
    let etf = etfs::table
        .filter(etfs::ticker.eq(&ticker))
        .first::<Etf>(conn)
        .optional()?;

    if let Some(etf) = etf {
        return Ok(etf.id);
    }

    let asset = diesel::insert_into(assets::table)
        .values(&NewAsset { kind: "etf" })
        .get_result::<Asset>(conn)?;

    let etf = diesel::insert_into(etfs::table)
        .values(&NewEtf {
            id: asset.id,
            ticker,
        })
        .get_result::<Etf>(conn)?;

    Ok(etf.id)
}
