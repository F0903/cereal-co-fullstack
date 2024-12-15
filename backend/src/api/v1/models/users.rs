use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password_plain: String,
}
