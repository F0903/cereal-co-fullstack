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
use rocket::{http::CookieJar, serde::json::Json, State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[post("/signup", format = "json", data = "<data>")]
pub async fn signup(data: Json<UserForm>) -> ApiResult<MessageObject> {
    //TODO
    ApiResponse::success().into_ok()
}

#[post("/login", format = "json", data = "<data>")]
pub async fn login(
    db: &State<DatabaseConnection>,
    data: Json<UserForm>,
    cookies: &CookieJar<'_>,
) -> ApiResult<MessageObject> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&data.username))
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::unauthorized())?
        .ok_or(ApiResponse::unauthorized())?;

    let hashed_password = hex::encode(digest(&SHA256, data.password_plain.as_bytes()));

    if hashed_password != user.password_hash {
        return ApiResponse::unauthorized().into_error();
    }

    let token = encode_jwt_token(user.id).map_err(|_| ApiResponse::internal_error())?;
    cookies.add((AUTH_COOKIE_NAME, token));

    ApiResponse::success().into_ok()
}

#[get("/logout")]
pub async fn logout(_jwt: JWT, cookies: &CookieJar<'_>) -> ApiResult<MessageObject> {
    cookies.remove(AUTH_COOKIE_NAME);
    ApiResponse::success().into_ok()
}
