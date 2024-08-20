use dotenvy_macro::dotenv;
use leptos::{error::Result, server_fn::request, use_context, RwSignal, Signal, SignalGet};
use leptos_query::{create_query, QueryKey, QueryOptions, QueryResult, QueryScope, RefetchFn};
use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use shared::{params::user::LoginParams, response::auth::LoginResponse};
use thiserror::Error;

// All query keys
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LoginQueryKey;

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub enum ApiError {
    #[error("Json error: {0}")]
    JsonError(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(String),
    #[error("Token does not exist.")]
    NoTokenError,
    #[error("401 Unauthorized")]
    Unauthorized,
    #[error("Undefined")]
    Undefined,
}

#[derive(Default, Clone)]
pub struct Api {
    client: Client,
    token: Option<String>,
}

impl Api {
    fn host() -> String {
        dotenv!("API_HOST").to_string()
    }

    /// Get api fetch
    ///
    /// `uri`: api endpoint
    /// `auth`: require authentification
    async fn get<T>(uri: &str, auth: Option<bool>) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let api = use_context::<RwSignal<Api>>()
            .expect("Provide api context.")
            .get();
        let mut request = api.client.get(Api::host() + uri);

        // add Authorization header
        if auth.is_some() {
            if let Some(token) = api.token {
                request = request.header("Authorization", format!("Bearer {}", token));
            } else {
                return None; // replace
            }
        }

        let response = request.send().await;

        if let Ok(result) = response {
            let result = result.json().await;
            result.ok()
        } else {
            None
        }
    }

    /// Post api fetch
    ///
    /// `uri`: api endpoint
    /// `auth`: require authentification
    /// `body`: request body
    async fn post<T, P>(uri: &str, auth: Option<bool>, body: Option<P>) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let api = use_api().get();
        let mut request = api.client.post(Api::host() + uri);

        // add Authorization header
        if auth.is_some() {
            if let Some(token) = api.token {
                request = request.header("Authorization", format!("Bearer {}", token));
            } else {
                return Err(ApiError::NoTokenError); // replace
            }
        }

        // add body
        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await;

        match response {
            Ok(result) => match result.status().as_u16() {
                200 => {
                    let result = result.json().await;

                    match result {
                        Ok(result) => Ok(result),
                        Err(e) => Err(ApiError::JsonError(e.to_string())),
                    }
                }
                401 => Err(ApiError::Unauthorized),
                _ => Err(ApiError::Undefined),
            },
            Err(e) => Err(ApiError::ReqwestError(e.to_string())),
        }
    }

    pub fn login(
        &mut self,
        body: LoginParams,
    ) -> QueryResult<Result<LoginResponse, ApiError>, impl RefetchFn> {
        let query = create_query(
            move |_: LoginQueryKey| {
                let body = body.clone();
                async move {
                    Api::post::<LoginResponse, LoginParams>("/auth/login", None, Some(body)).await
                }
            },
            QueryOptions::default(),
        )
        .use_query(move || LoginQueryKey);

        // set token
        if let Some(response) = &query.data.get() {
            if let Ok(data) = response {
                self.token = Some(data.token.clone());
            }
        }

        query
    }
}

pub fn use_api() -> RwSignal<Api> {
    use_context::<RwSignal<Api>>().expect("Please provide the Api")
}

// fn get_data<T>(query: &QueryResult<T, impl RefetchFn>) -> T
// where
//     T: Clone,
// {
//     query.data.get().unwrap()
// }
