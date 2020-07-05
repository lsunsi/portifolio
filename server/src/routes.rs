mod healthz;
mod import_etf_prices;
mod import_trades;
mod import_treasury_prices;
mod portfolio_position;

use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(healthz::healthz)
        .service(import_trades::post)
        .service(import_etf_prices::post)
        .service(import_treasury_prices::post)
        .service(portfolio_position::get);
}
