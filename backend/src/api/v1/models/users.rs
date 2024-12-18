use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ChangePasswordForm {
    pub old_password_plain: String,
    pub new_password_plain: String,
}

#[derive(Deserialize)]
pub struct UserSignupForm {
    pub mail: String,
    pub password_plain: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct UserLoginForm {
    pub mail: String,
    pub password_plain: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub mail: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_admin: bool,
}
