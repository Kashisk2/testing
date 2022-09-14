use std::fmt::{Display, Formatter, Result as FmtResult};

use actix_web::{http, web, Error, HttpRequest, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};

use crate::detect;
use crate::detect::AddressChain;
use crate::logos;

#[derive(Debug, Serialize)]
struct PubkeyChainResponse {
    pub chain: detect::AddressChain,
    pub logo: String,
}

#[derive(Debug, Deserialize)]
pub struct SupportedParams {
    size: Option<usize>,
}

pub async fn healthcheck(
    _req: HttpRequest,
    _params: web::Query<SupportedParams>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_pubkey_chain(
    req: HttpRequest,
    params: web::Query<SupportedParams>,
    b58_pubkey: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let default = http::header::HeaderValue::from_static("application/xml");
    let resp_codec = req
        .headers()
        .get("Accept")
        .unwrap_or(&default)
        .to_str()
        .unwrap_or("application/xml");

    let addr_chain = detect::detect_address(&b58_pubkey);
    match addr_chain {
        AddressChain::Invalid => Err(ServiceError {
            msg: "public key invalid".to_string(),
            status: 400,
        }
        .into()),
        AddressChain::Unknown => Err(ServiceError {
            msg: "public key not found or supported".to_string(),
            status: 400,
        }
        .into()),
        _ => {
            // Get sized or non-sized logo
            let logo_svg = match params.size {
                Some(size) => logos::get_sized_logo(&addr_chain, size),
                None => logos::get_logo(&addr_chain).to_string(),
            };

            match resp_codec {
                "application/json" => Ok(HttpResponse::Ok().json(PubkeyChainResponse {
                    chain: addr_chain,
                    logo: logo_svg,
                })),
                _ => Ok(HttpResponse::Ok()
                    .insert_header(http::header::ContentType::html())
                    .body(logo_svg)),
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct ServiceError {
    msg: String,
    status: u16,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for ServiceError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.msg });
        HttpResponse::build(http::StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}
