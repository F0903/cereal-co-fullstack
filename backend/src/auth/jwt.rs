use crate::{
    api::v1::{ApiError, ApiResponse},
    utils::generic_result::GenericResult,
};
use chrono::Utc;
use dotenv_codegen::dotenv;
use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Response,
};

const JWT_ALGORITHM: Algorithm = Algorithm::HS512;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i32,
    exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

impl<'r> FromRequest<'r> for JWT {
    type Error = ApiResponse<ApiError>;

    async fn from_request<'life0, 'async_trait>(
        request: &'r rocket::Request<'life0>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = rocket::request::Outcome<Self, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'r: 'async_trait,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let headers = request.headers();
        let authorization = headers.get_one("Authorization");

        match authorization {
            Some(key) => match decode_jwt(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match err {
                    ErrorKind::ExpiredSignature => {
                        let response = Response {
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - Expired Token"
                            )),
                        };
                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(
                                serde_json::to_string(&response).unwrap(),
                            ),
                        ))
                    }
                    ErrorKind::InvalidToken => {
                        let response = Response {
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - Invalid Token"
                            )),
                        };
                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(
                                serde_json::to_string(&response).unwrap(),
                            ),
                        ))
                    }
                    _ => {
                        let response = Response {
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - {}",
                                err
                            )),
                        };
                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(
                                serde_json::to_string(&response).unwrap(),
                            ),
                        ))
                    }
                },
            },
            None => {
                let response = ApiResponse::unauthorized();
                Outcome::Error((Status::Unauthorized, response))
            }
        }
    }
}

pub fn encode_jwt(id: i32) -> GenericResult<String> {
    let secret: &str = dotenv!("JWT_SECRET");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize,
    };

    let header = Header::new(JWT_ALGORITHM);
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token_header: &str) -> Result<Claims, jsonwebtoken::errors::ErrorKind> {
    let secret: &str = dotenv!("JWT_SECRET");

    let token = token_header.trim_start_matches("Bearer").trim();

    let token = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(JWT_ALGORITHM),
    )
    .map_err(|e| e.kind().to_owned())?;

    Ok(token.claims)
}
