use serde::{Deserialize, Serialize};

use crate::models::_entities::{pages, users};

use super::user::UserAvatarResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct DetailPageResponse {
    pub id: i32,
    pub name: String,
    pub content: Option<String>,
    pub collaborators: Vec<UserAvatarResponse>,
}

impl DetailPageResponse {
    pub fn new(page: &pages::Model, collaborators: Vec<users::Model>) -> Self {
        let mut users = Vec::new();
        for user in collaborators.iter() {
            users.push(UserAvatarResponse::new(user));
        }

        Self {
            id: page.id.clone(),
            name: page.name.clone(),
            content: page.content.clone(),
            collaborators: users,
        }
    }
}
