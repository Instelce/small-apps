use axum::debug_handler;
use cookie::{Cookie, SameSite};
use loco_rs::{controller::bad_request, prelude::*};
use serde::{Deserialize, Serialize};
use shared::{params::user::LoginParams, response::auth::LoginResponse};
use tower_cookies::{cookie::CookieBuilder, Cookies};

use crate::{
    errors::ResponseError,
    mailers::auth::AuthMailer,
    models::{_entities::users, users::RegisterParams},
    views::FromModel,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            return format::json(());
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;

    format::json(())
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(
    State(ctx): State<AppContext>,
    Json(params): Json<VerifyParams>,
) -> Result<Response> {
    let user = users::Model::find_by_verification_token(&ctx.db, &params.token).await?;

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.pid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await?;
        tracing::info!(pid = user.pid.to_string(), "user verified");
    }

    format::json(())
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return format::json(());
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user).await?;

    format::json(())
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let Ok(user) = users::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(
    // cookies: Cookies,
    State(ctx): State<AppContext>,
    Json(params): Json<LoginParams>,
) -> Result<Response> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await;

    if let Err(_) = user {
        return Ok(ResponseError::bad_request(
            "User with this e-mail adress doesn't exist.",
        ));
    }

    let user = user.unwrap();

    let valid = user.verify_password(&params.password);

    if !valid {
        return Ok(ResponseError::unauthorized(
            "This combination of e-mail address and password is incorrect.",
        ));
    }

    // let mut cookie = Cookie::new("token", user.api_key.clone());
    // cookie.set_http_only(true);
    // // cookie.set_path("/");
    // cookie.set_same_site(Some(cookie::SameSite::None));
    // cookie.set_secure(Some(true));
    // cookies.add(cookie);

    format::render()
        .cookies(&[CookieBuilder::new("token", user.api_key.clone())
            .same_site(SameSite::None)
            .secure(true)
            .path("/")
            .http_only(true)
            .build()])?
        .json(LoginResponse::new(&user))
}

async fn already_login(cookies: Cookies, State(ctx): State<AppContext>) -> Result<Response> {
    if let Some(token) = cookies.get("token") {
        let user = users::Model::find_by_api_key(&ctx.db, token.value()).await?;
        return format::json(LoginResponse::new(&user));
    }

    format::json(())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("auth")
        .add("/register", post(register))
        .add("/verify", post(verify))
        .add("/login", post(login))
        .add("/already-login", get(already_login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
}
