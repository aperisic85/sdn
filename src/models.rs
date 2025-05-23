use serde::{Deserialize, Serialize};

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}
