#[actix_web::get("/healthz")]
pub async fn healthz() -> &'static str {
    "ok"
}
