use axum::debug_handler;
use loco_rs::prelude::*;

use crate::{
    models::_entities::{user_pages, users},
    views::user::CurrentResponse,
};

async fn load_user(ctx: &AppContext, id: &i32) -> Result<users::Model> {
    users::Model::find_by_id(&ctx.db, id)
        .await
        .or_else(|_| Err(Error::NotFound))
}

#[debug_handler]
async fn current(
    auth: auth::ApiToken<users::Model>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.user.pid.to_string()).await?;
    format::json(CurrentResponse::new(&user))
}

#[debug_handler]
async fn current_user_pages(
    auth: auth::ApiToken<users::Model>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_user(&ctx, &auth.user.id).await?;
    let pages = item.find_related(user_pages::Entity).all(&ctx.db).await?;
    format::json(pages)
}

#[debug_handler]
async fn pages(
    _auth: auth::ApiToken<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_user(&ctx, &id).await?;
    let pages = item.find_related(user_pages::Entity).all(&ctx.db).await?;
    format::json(pages)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("user")
        .add("/current", get(current))
        .add("/current/pages", get(current_user_pages))
        .add("/:id/pages", get(pages))
}
