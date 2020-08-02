use crate::schema::{assets, etfs, treasury_bonds};
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
#[derive(Queryable, Clone)]
pub struct TreasuryBond {
    pub id: i32,
    kind: String,
    pub maturity_date: NaiveDate,
    pub key: String,
}

#[allow(dead_code)]
#[derive(Queryable, Clone)]
pub struct Etf {
    pub id: i32,
    kind: String,
    pub ticker: String,
}

#[derive(Insertable)]
#[table_name = "assets"]
struct NewAsset {
    kind: &'static str,
}

#[derive(Insertable)]
#[table_name = "treasury_bonds"]
struct NewTreasuryBond {
    id: i32,
    maturity_date: NaiveDate,
    key: &'static str,
}

#[derive(Insertable)]
#[table_name = "etfs"]
struct NewEtf<'a> {
    id: i32,
    ticker: &'a str,
}

#[derive(Clone)]
pub enum Assetable {
    TreasuryBond(TreasuryBond),
    Etf(Etf),
}

pub fn register_treasury_bond_asset(
    conn: &PgConnection,
    key: &'static str,
    maturity_date: NaiveDate,
) -> QueryResult<i32> {
    let treasury_bond = treasury_bonds::table
        .filter(treasury_bonds::maturity_date.eq(maturity_date))
        .first::<TreasuryBond>(conn)
        .optional()?;

    if let Some(treasury_bond) = treasury_bond {
        return Ok(treasury_bond.id);
    }

    let asset = diesel::insert_into(assets::table)
        .values(&NewAsset {
            kind: "treasury_bond",
        })
        .get_result::<Asset>(conn)?;

    let treasury_bond = diesel::insert_into(treasury_bonds::table)
        .values(&NewTreasuryBond {
            id: asset.id,
            maturity_date,
            key,
        })
        .get_result::<TreasuryBond>(conn)?;

    Ok(treasury_bond.id)
}

pub fn register_etf_asset(conn: &PgConnection, ticker: &str) -> QueryResult<i32> {
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

pub fn retrieve_assetables(conn: &PgConnection, asset_ids: &[i32]) -> QueryResult<Vec<Assetable>> {
    let etfs = etfs::table
        .filter(etfs::id.eq_any(asset_ids))
        .load::<Etf>(conn)?;

    let treasury_bonds = treasury_bonds::table
        .filter(treasury_bonds::id.eq_any(asset_ids))
        .load::<TreasuryBond>(conn)?;

    Ok(asset_ids
        .iter()
        .map(|asset_id| {
            etfs.iter()
                .find_map(|etf| {
                    if etf.id == *asset_id {
                        Some(Assetable::Etf(etf.clone()))
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    treasury_bonds.iter().find_map(|treasury_bond| {
                        if treasury_bond.id == *asset_id {
                            Some(Assetable::TreasuryBond(treasury_bond.clone()))
                        } else {
                            None
                        }
                    })
                })
                .unwrap()
        })
        .collect())
}
