use crate::schema::{asset_prices, trades};
use bigdecimal::{BigDecimal, Zero};
use chrono::{Duration, NaiveDate};
use diesel::{dsl::sql, prelude::*, sql_types::Numeric};

pub struct PortfolioAmount {
    pub gross_total: BigDecimal,
    pub invested: BigDecimal,
    pub date: NaiveDate,
}

pub fn run(
    conn: &PgConnection,
    portfolio_id: i32,
    today: NaiveDate,
) -> QueryResult<Vec<PortfolioAmount>> {
    let trades = trades::table
        .select((
            trades::asset_id,
            trades::date,
            sql::<Numeric>("sum(quantity * price)"),
            sql::<Numeric>("sum(quantity)"),
        ))
        .filter(trades::portfolio_id.eq(portfolio_id))
        .group_by((trades::asset_id, trades::date))
        .order(trades::date)
        .load::<Trade>(conn)?;

    let dates = match trades.first() {
        Some(first_trade) => date_series(first_trade.date, today),
        None => return Ok(vec![]),
    };

    let mut asset_ids = trades
        .iter()
        .map(|trade| trade.asset_id)
        .collect::<Vec<_>>();

    asset_ids.sort_unstable();
    asset_ids.dedup();

    let prices = asset_prices::table
        .select((
            asset_prices::asset_id,
            asset_prices::date,
            asset_prices::price,
        ))
        .filter(asset_prices::asset_id.eq_any(&asset_ids))
        .filter(asset_prices::date.eq_any(&dates))
        .order((asset_prices::asset_id, asset_prices::date.desc()))
        .load::<Price>(conn)?;

    let mut portfolio_amounts = vec![];

    let zero = BigDecimal::zero();
    for date in dates {
        let mut gross_total = BigDecimal::zero();
        let mut invested = BigDecimal::zero();

        for asset_id in &asset_ids {
            let price = prices
                .iter()
                .filter(|price| price.asset_id == *asset_id)
                .skip_while(|price| price.date > date)
                .map(|price| &price.price)
                .next()
                .unwrap_or(&zero);

            for date_trade in trades
                .iter()
                .filter(|trade| trade.date <= date && trade.asset_id == *asset_id)
            {
                gross_total += &date_trade.quantity * price;
                invested += &date_trade.amount;
            }
        }

        portfolio_amounts.push(PortfolioAmount {
            gross_total,
            invested,
            date,
        });
    }

    Ok(portfolio_amounts)
}

fn date_series(first_date: NaiveDate, last_date: NaiveDate) -> Vec<NaiveDate> {
    let mut date = first_date;
    let mut dates = vec![];

    loop {
        dates.push(date);
        date += Duration::days(1);
        if date > last_date {
            break;
        }
    }

    dates
}

#[derive(Queryable)]
struct Trade {
    asset_id: i32,
    date: NaiveDate,
    amount: BigDecimal,
    quantity: BigDecimal,
}

#[derive(Queryable)]
struct Price {
    asset_id: i32,
    date: NaiveDate,
    price: BigDecimal,
}
