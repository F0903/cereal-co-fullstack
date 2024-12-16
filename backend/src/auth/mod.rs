mod jwt;

pub use jwt::{encode_jwt_token, JWT};

pub const AUTH_COOKIE_NAME: &str = "Auth";
