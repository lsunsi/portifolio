use crate::models::assets::{retrieve_assetables, Assetable};
use crate::models::prices::latest_prices;
use crate::schema::trades;
use bigdecimal::{BigDecimal, Zero};
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use itertools::Itertools;

pub struct AssetPosition {
    pub assetable: Assetable,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub amount: BigDecimal,
}

pub struct PortfolioPosition {
    pub amount: BigDecimal,
    pub assets: Vec<AssetPosition>,
}

pub fn position(
    conn: &PgConnection,
    portfolio_id: i32,
    date: NaiveDate,
) -> QueryResult<PortfolioPosition> {
    let trades = trades::table
        .select((trades::asset_id, trades::quantity))
        .filter(trades::portfolio_id.eq(portfolio_id))
        .filter(trades::date.le(date))
        .order(trades::asset_id)
        .load::<(i32, BigDecimal)>(conn)?;

    let mut asset_ids = vec![];
    let mut quantities = vec![];

    for (asset_id, group) in &trades.into_iter().group_by(|(asset_id, _)| *asset_id) {
        quantities.push(group.map(|(_, quantity)| quantity).sum::<BigDecimal>());
        asset_ids.push(asset_id);
    }

    let prices = latest_prices(conn, &asset_ids, date)?;
    let assetables = retrieve_assetables(conn, &asset_ids)?;

    let mut portfolio_amount = BigDecimal::zero();
    let mut assets = vec![];

    for i in 0..quantities.len() {
        let price = prices[i].clone();
        let quantity = quantities[i].clone();
        let amount = &price * &quantity;

        portfolio_amount += &amount;

        assets.push(AssetPosition {
            assetable: assetables[i].clone(),
            quantity,
            amount,
            price,
        })
    }

    Ok(PortfolioPosition {
        amount: portfolio_amount,
        assets,
    })
}
