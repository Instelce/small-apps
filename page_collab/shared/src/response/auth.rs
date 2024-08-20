use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}
