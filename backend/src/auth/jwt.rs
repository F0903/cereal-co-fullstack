use crate::{
    api::v1::{ApiError, ApiResponse},
    utils::generic_result::GenericResult,
};
use chrono::Utc;
use dotenv_codegen::dotenv;
use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Response,
};

//TODO

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ApiResponse<ApiError>;

    async fn from_request(req: &'r request::Request<'_>) -> request::Outcome<Self, Self::Error> {
        let headers = req.headers();
        let authorization = headers.get_one("Authorization");

        match authorization {
            Some(key) => match decode_jwt(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(_) => {
                    let response = ApiResponse::unauthorized();
                    Outcome::Error((Status::Unauthorized, response))
                }
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
