mod jwt;

pub use jwt::{decode_jwt_token, encode_jwt_token, JWT};

pub const AUTH_COOKIE_NAME: &str = "Auth";
