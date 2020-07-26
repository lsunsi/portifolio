mod healthz;
mod import_etfs_prices;
mod import_trades;
mod import_treasury_bonds_prices;
mod portfolio_amounts;
mod portfolio_position;
mod transactions;

use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(healthz::healthz)
        .service(import_trades::post)
        .service(import_etfs_prices::post)
        .service(import_treasury_bonds_prices::post)
        .service(portfolio_position::get)
        .service(portfolio_amounts::get)
        .service(transactions::get);
}
