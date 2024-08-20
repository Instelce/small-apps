use shared::response::auth::LoginResponse;

use crate::models::_entities::users;

use super::FromModel;

impl FromModel<users::Model> for LoginResponse {
    #[must_use]
    fn new(model: &users::Model) -> Self {
        Self {
            token: model.api_key.to_string(),
            pid: model.pid.to_string(),
            name: model.name.clone(),
            is_verified: model.email_verified_at.is_some(),
        }
    }
}
