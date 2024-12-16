use super::{
    api_response::MessageObject,
    api_result::{ApiResult, ApiResultIntoError, ApiResultIntoOk},
    models::UserForm,
    ApiResponse,
};
use crate::{
    auth::{encode_jwt_token, AUTH_COOKIE_NAME, JWT},
    entities::user,
};
use ring::digest::{digest, SHA256};
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
    let user_model = user::ActiveModel {
        is_admin: Set(false.into()),
        mail: Set(user_form.mail.to_ascii_lowercase()),
        password_hash: Set(hex::encode(digest(
            &SHA256,
            user_form.password_plain.as_bytes(),
        ))),
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

    // Be aware that SHA256 is not optimal for this.
    let hashed_password = hex::encode(digest(&SHA256, user_form.password_plain.as_bytes()));
    if hashed_password != user.password_hash {
        return ApiResponse::unauthorized().into_error();
    }

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
