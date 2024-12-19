use super::{
    api_response::MessageObject,
    api_result::{ApiResult, ApiResultIntoOk},
    models::{ChangePasswordForm, UserLoginForm, UserResponse, UserSignupForm},
    ApiResponse,
};
use crate::{
    auth::{encode_jwt_token, AUTH_COOKIE_NAME, DEFAULT_TOKEN_EXPIRY_SECONDS, JWT},
    entities::user,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rocket::{
    http::{Cookie, CookieJar},
    serde::json::Json,
    time::Duration,
    State,
};
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[post("/signup", format = "json", data = "<user_form>")]
pub async fn signup(
    db: &State<DatabaseConnection>,
    user_form: Json<UserSignupForm>,
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
        name: Set(user_form.name.clone()),
        address: Set(user_form.address.clone()),
        phone: Set(user_form.phone.clone()),
        ..Default::default()
    };

    // Will automatically return conflict if a record exists.
    user::Entity::insert(user_model).exec(db.inner()).await?;

    ApiResponse::success().into_ok()
}

#[post("/login", format = "json", data = "<user_form>")]
pub async fn login(
    db: &State<DatabaseConnection>,
    user_form: Json<UserLoginForm>,
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
    cookies.add(
        Cookie::build((AUTH_COOKIE_NAME, token))
            .max_age(Duration::seconds_f64(DEFAULT_TOKEN_EXPIRY_SECONDS as f64))
            .http_only(true),
    );

    ApiResponse::success().into_ok()
}

#[post("/logout")]
pub async fn logout(_jwt: JWT, cookies: &CookieJar<'_>) -> ApiResult<MessageObject> {
    // To logout, we simply tell the client to remove the cookie. Although be aware that the token is still valid until expiry.
    cookies.remove(Cookie::build(AUTH_COOKIE_NAME));
    ApiResponse::success().into_ok()
}

#[get("/get_user")]
pub async fn get_logged_in_user(
    jwt: JWT,
    db: &State<DatabaseConnection>,
) -> ApiResult<UserResponse> {
    let user = user::Entity::find_by_id(jwt.claims.sub)
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::unauthorized())?
        .ok_or(ApiResponse::unauthorized())?;

    let user_info = UserResponse {
        is_admin: user.is_admin != 0,
        mail: user.mail,
        name: user.name,
        address: user.address,
        phone: user.phone,
    };

    ApiResponse::ok(user_info).into_ok()
}

#[post("/change_password", format = "json", data = "<passwords>")]
pub async fn change_password(
    jwt: JWT,
    db: &State<DatabaseConnection>,
    passwords: Json<ChangePasswordForm>,
) -> ApiResult<MessageObject> {
    let user = user::Entity::find_by_id(jwt.claims.sub)
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::unauthorized())?
        .ok_or(ApiResponse::unauthorized())?;

    let argon2 = Argon2::default();
    let salt = SaltString::generate(OsRng);

    // Check that the old password is actually correct.
    argon2
        .verify_password(
            passwords.old_password_plain.as_bytes(),
            &PasswordHash::new(&user.password_hash).map_err(|_| ApiResponse::bad_request())?,
        )
        .map_err(|_| ApiResponse::unauthorized())?;

    // Get the existing record and set the new password.
    let mut user_model = user.into_active_model();
    user_model.password_hash = Set(argon2
        .hash_password(passwords.new_password_plain.as_bytes(), &salt)
        .map_err(|_| ApiResponse::internal_error())?
        .to_string());

    // Insert updated model
    user_model
        .update(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;

    ApiResponse::success().into_ok()
}
