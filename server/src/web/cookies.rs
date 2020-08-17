use actix_http::{http::HeaderMap, Payload};
use actix_web::{body::Body, cookie, http::header, FromRequest, HttpRequest, HttpResponse};
use futures::future::{ready, Ready};
use std::convert::TryFrom;

const PORTFOLIO_ID: &'static str = "portfolio-id";

pub fn portfolio_id_cookie(id: i32) -> cookie::Cookie<'static> {
    let mut cookie = cookie::Cookie::new(PORTFOLIO_ID, id.to_string());
    cookie.set_same_site(cookie::SameSite::Strict);
    cookie.set_domain("portifolio.lsunsi.com");
    cookie.set_http_only(true);
    cookie.make_permanent();
    cookie
}

pub struct PortfolioId(pub i32);

impl TryFrom<&HeaderMap> for PortfolioId {
    type Error = ();

    fn try_from(map: &HeaderMap) -> Result<PortfolioId, ()> {
        let value = map.get(header::COOKIE).ok_or(())?;
        let str = value.to_str().or(Err(()))?;

        for s in str.split(";") {
            let cookie = cookie::Cookie::parse(s).or(Err(()))?;

            if cookie.name() == PORTFOLIO_ID {
                let id = cookie.value().parse::<i32>().or(Err(()))?;
                return Ok(PortfolioId(id));
            }
        }

        Err(())
    }
}

impl FromRequest for PortfolioId {
    type Future = Ready<Result<PortfolioId, Self::Error>>;
    type Error = HttpResponse<Body>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(
            PortfolioId::try_from(req.headers()).map_err(|()| HttpResponse::BadRequest().finish()),
        )
    }
}
