use crate::models::assets::{Etf, TreasuryBond};
use crate::schema::{etfs, portfolios, trades, treasury_bonds};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use itertools::Itertools;

#[allow(dead_code)]
#[derive(Queryable)]
struct Portfolio {
    id: i32,
}

#[allow(dead_code)]
#[derive(Queryable)]
struct Trade {
    id: i32,
    portfolio_id: i32,
    asset_id: i32,
    date: NaiveDate,
    quantity: BigDecimal,
    price: BigDecimal,
}

#[derive(Insertable)]
#[table_name = "trades"]
struct NewTrade<'a> {
    portfolio_id: i32,
    asset_id: i32,
    date: &'a NaiveDate,
    quantity: &'a BigDecimal,
    price: &'a BigDecimal,
}

pub struct EtfTrade {
    pub ticker: String,
    pub date: NaiveDate,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
}

pub struct TreasuryBondTrade {
    pub key: String,
    pub maturity: NaiveDate,
    pub date: NaiveDate,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
}

pub fn register_trades(
    conn: &PgConnection,
    etf_trades: &[EtfTrade],
    treasury_bond_trades: &[TreasuryBondTrade],
) -> QueryResult<i32> {
    conn.transaction(|| {
        let portfolio = diesel::insert_into(portfolios::table)
            .default_values()
            .get_result::<Portfolio>(conn)?;

        let mut new_trades = vec![];

        for (ticker, trades) in &etf_trades
            .iter()
            .sorted_by_key(|t| &t.ticker)
            .group_by(|t| &t.ticker)
        {
            let etf = etfs::table
                .filter(etfs::ticker.eq(ticker))
                .first::<Etf>(conn)?;

            for trade in trades {
                new_trades.push(NewTrade {
                    portfolio_id: portfolio.id,
                    asset_id: etf.id,
                    quantity: &trade.quantity,
                    price: &trade.price,
                    date: &trade.date,
                });
            }
        }

        for ((key, maturity_date), trades) in &treasury_bond_trades
            .iter()
            .sorted_by_key(|t| (&t.key, t.maturity))
            .group_by(|t| (&t.key, t.maturity))
        {
            let treasury_bond = treasury_bonds::table
                .filter(treasury_bonds::maturity_date.eq(maturity_date))
                .filter(treasury_bonds::key.eq(key))
                .first::<TreasuryBond>(conn)?;

            for trade in trades {
                new_trades.push(NewTrade {
                    portfolio_id: portfolio.id,
                    asset_id: treasury_bond.id,
                    quantity: &trade.quantity,
                    price: &trade.price,
                    date: &trade.date,
                });
            }
        }

        diesel::insert_into(trades::table)
            .values(&new_trades)
            .execute(conn)?;

        Ok(portfolio.id)
    })
}
