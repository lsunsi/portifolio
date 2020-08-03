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

pub fn register_trades(
    conn: &PgConnection,
    etf_trades: Vec<(String, NaiveDate, BigDecimal, BigDecimal)>,
    treasury_bond_trades: Vec<(NaiveDate, NaiveDate, BigDecimal, BigDecimal)>,
) -> QueryResult<i32> {
    conn.transaction(|| {
        let portfolio = diesel::insert_into(portfolios::table)
            .default_values()
            .get_result::<Portfolio>(conn)?;

        let mut new_trades = vec![];

        for (ticker, trades) in &etf_trades
            .iter()
            .sorted_by_key(|et| &et.0)
            .group_by(|et| &et.0)
        {
            let etf = etfs::table
                .filter(etfs::ticker.eq(ticker))
                .first::<Etf>(conn)?;

            for (_, date, price, quantity) in trades {
                new_trades.push(NewTrade {
                    portfolio_id: portfolio.id,
                    asset_id: etf.id,
                    quantity,
                    price,
                    date,
                });
            }
        }

        for (maturity_date, trades) in &treasury_bond_trades
            .iter()
            .sorted_by_key(|et| &et.0)
            .group_by(|et| &et.0)
        {
            let treasury_bond = treasury_bonds::table
                .filter(treasury_bonds::maturity_date.eq(maturity_date))
                .filter(treasury_bonds::key.eq("LFT"))
                .first::<TreasuryBond>(conn)?;

            for (_, date, price, quantity) in trades {
                new_trades.push(NewTrade {
                    portfolio_id: portfolio.id,
                    asset_id: treasury_bond.id,
                    quantity,
                    price,
                    date,
                });
            }
        }

        diesel::insert_into(trades::table)
            .values(&new_trades)
            .execute(conn)?;

        Ok(portfolio.id)
    })
}
