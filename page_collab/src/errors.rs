use axum::body::Body;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseError {
    #[serde(skip)]
    pub status: u16,
    pub error: String,
    pub description: String,
}

impl ResponseError {
    pub fn new(status: u16, description: impl Into<String>) -> Self {
        Self {
            status,
            error: status_string(&status),
            description: description.into(),
        }
    }

    pub fn bad_request(description: impl Into<String>) -> Response {
        ResponseError::new(400, description).into()
    }

    pub fn unauthorized(description: impl Into<String>) -> Response {
        ResponseError::new(401, description).into()
    }

    pub fn not_found(description: impl Into<String>) -> Response {
        ResponseError::new(404, description).into()
    }
}

impl Into<Response> for ResponseError {
    fn into(self) -> Response {
        Response::builder()
            .status(self.status)
            .body(Body::new(serde_json::to_string(&self).unwrap()))
            .unwrap()
    }
}

fn status_string(status: &u16) -> String {
    match status {
        404 => "Not found".into(),
        401 => "Unauthorized".into(),
        400 => "Bad request".into(),
        _ => "Bad request".into(),
    }
}
