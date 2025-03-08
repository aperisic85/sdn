use serde::{Deserialize, Serialize};

struct User {
    id: i32,
    name: String,
    email: String,
    password_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserRequest {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserResponse {
    id: i32,
    name: String,
    email: String,
}
