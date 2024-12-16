use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserForm {
    pub mail: String,
    pub password_plain: String,
}
