use serde::{Deserialize, Serialize};

// The form model to receive
#[derive(Deserialize)]
pub struct UserForm {
    pub mail: String,
    pub password_plain: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

// The form model to send
#[derive(Serialize)]
pub struct UserInfo {
    pub mail: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_admin: bool,
}
