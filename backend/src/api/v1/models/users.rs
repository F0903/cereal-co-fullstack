use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserForm {
    pub mail: String,
    pub password_plain: String,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub mail: String,
    pub decorative_username: String, // For now, username is just the prefix of the mail, and purely "decorative"
    pub is_admin: bool,
}
