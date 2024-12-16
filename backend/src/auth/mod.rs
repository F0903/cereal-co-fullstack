mod jwt;

pub use jwt::{encode_jwt_token, JWT};

pub const AUTH_COOKIE_NAME: &str = "Auth";
pub const DEFAULT_TOKEN_EXPIRY_SECONDS: i64 = chrono::Duration::hours(6).num_seconds();
