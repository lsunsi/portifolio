mod healthz;
mod import_treasury_prices;

use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(healthz::healthz)
        .service(import_treasury_prices::post);
}
