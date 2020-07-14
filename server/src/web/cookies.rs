use actix_http::{http::HeaderMap, Payload};
use actix_web::{body::Body, cookie::Cookie, http::header, FromRequest, HttpRequest, HttpResponse};
use futures::future::{ready, Ready};
use std::convert::TryFrom;

const PORTFOLIO_ID: &'static str = "portfolio-id";

pub fn portfolio_id_cookie(id: i32) -> Cookie<'static> {
    Cookie::new(PORTFOLIO_ID, id.to_string())
}

pub struct PortfolioId(pub i32);

impl TryFrom<&HeaderMap> for PortfolioId {
    type Error = ();

    fn try_from(map: &HeaderMap) -> Result<PortfolioId, ()> {
        let value = map.get(header::COOKIE).ok_or(())?;
        let str = value.to_str().or(Err(()))?;

        for s in str.split(";") {
            let cookie = Cookie::parse(s).or(Err(()))?;

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
        ready(PortfolioId::try_from(req.headers()).map_err(|()| HttpResponse::Forbidden().finish()))
    }
}
