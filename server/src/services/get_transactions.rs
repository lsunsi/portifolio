use crate::{
    models::{retrieve_assetables, Assetable},
    schema::trades,
};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;

pub struct Transaction {
    pub assetable: Assetable,
    pub date: NaiveDate,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub amount: BigDecimal,
}

pub fn run(conn: &PgConnection, portfolio_id: i32) -> QueryResult<Vec<Transaction>> {
    let trades = trades::table
        .select((
            trades::asset_id,
            trades::date,
            trades::price,
            trades::quantity,
        ))
        .filter(trades::portfolio_id.eq(portfolio_id))
        .order((trades::date.desc(), trades::asset_id))
        .load::<Trade>(conn)?;

    let assetables = retrieve_assetables(
        conn,
        &trades
            .iter()
            .map(|trade| trade.asset_id)
            .collect::<Vec<_>>(),
    )?;

    Ok(trades
        .into_iter()
        .zip(assetables)
        .map(|(trade, assetable)| Transaction {
            amount: &trade.price * &trade.quantity,
            quantity: trade.quantity,
            price: trade.price,
            date: trade.date,
            assetable,
        })
        .collect())
}

#[derive(Queryable)]
struct Trade {
    asset_id: i32,
    date: NaiveDate,
    price: BigDecimal,
    quantity: BigDecimal,
}
