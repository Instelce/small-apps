use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}
