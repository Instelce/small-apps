use serde::{Deserialize, Serialize};

use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

/// Single user
#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl UserResponse {
    pub fn new(user: &users::Model) -> Self {
        Self {
            id: user.id.clone(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

/// List of users
#[derive(Debug, Deserialize, Serialize)]
pub struct UsersResponse(Vec<UserResponse>);

impl UsersResponse {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_vec(users: Vec<users::Model>) -> Self {
        Self(users.iter().map(|user| UserResponse::new(&user)).collect())
    }
}
