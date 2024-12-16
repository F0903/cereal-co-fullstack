use super::{
    api_response::MessageObject,
    api_result::{ApiResult, ApiResultIntoOk},
    models::UserForm,
    ApiResponse,
};
use crate::{
    auth::{encode_jwt_token, AUTH_COOKIE_NAME, JWT},
    entities::user,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rocket::{
    http::{Cookie, CookieJar},
    serde::json::Json,
    State,
};
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[post("/signup", format = "json", data = "<user_form>")]
pub async fn signup(
    db: &State<DatabaseConnection>,
    user_form: Json<UserForm>,
) -> ApiResult<MessageObject> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(OsRng);

    let user_model = user::ActiveModel {
        is_admin: Set(false.into()),
        mail: Set(user_form.mail.to_ascii_lowercase()),
        password_hash: Set(argon2
            .hash_password(user_form.password_plain.as_bytes(), &salt)
            .map_err(|_| ApiResponse::internal_error())?
            .to_string()),
        ..Default::default()
    };

    user::Entity::insert(user_model)
        .exec(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;

    ApiResponse::success().into_ok()
}

#[post("/login", format = "json", data = "<user_form>")]
pub async fn login(
    db: &State<DatabaseConnection>,
    user_form: Json<UserForm>,
    cookies: &CookieJar<'_>,
) -> ApiResult<MessageObject> {
    let user = user::Entity::find()
        .filter(user::Column::Mail.eq(&user_form.mail))
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::unauthorized())?
        .ok_or(ApiResponse::unauthorized())?;

    let argon2 = Argon2::default();
    argon2
        .verify_password(
            user_form.password_plain.as_bytes(),
            &PasswordHash::new(&user.password_hash).map_err(|_| ApiResponse::internal_error())?,
        )
        .map_err(|_| ApiResponse::unauthorized())?;

    let token = encode_jwt_token(&user).map_err(|_| ApiResponse::internal_error())?;
    cookies.add(Cookie::build((AUTH_COOKIE_NAME, token)).http_only(false /* turn this on later*/));

    ApiResponse::success().into_ok()
}

#[post("/logout")]
pub async fn logout(_jwt: JWT, cookies: &CookieJar<'_>) -> ApiResult<MessageObject> {
    println!("{:?}", _jwt);
    // To logout, we simply tell the client to remove the cookie. Although be aware that the token is still valid until expiry.
    cookies.remove(Cookie::build(AUTH_COOKIE_NAME));
    ApiResponse::success().into_ok()
}
