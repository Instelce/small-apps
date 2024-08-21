use axum::debug_handler;
use loco_rs::prelude::*;

use crate::{
    models::_entities::{pages, user_pages, users},
    views::{page::DetailPageResponse, user::CurrentResponse},
};

async fn load_user(ctx: &AppContext, id: &i32) -> Result<users::Model> {
    users::Model::find_by_id(&ctx.db, id)
        .await
        .or_else(|_| Err(Error::NotFound))
}

async fn count(State(ctx): State<AppContext>) -> Result<Response> {
    let count = users::Entity::find().all(&ctx.db).await?.len();
    format::json(count)
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
async fn pages(
    auth: auth::ApiToken<users::Model>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_user(&ctx, &auth.user.id).await?;
    let page_relations = item.find_related(user_pages::Entity).all(&ctx.db).await?;

    let mut pages = Vec::new();
    for relation in page_relations {
        let page = pages::Entity::find()
            .filter(pages::Column::Id.eq(relation.page_id))
            .one(&ctx.db)
            .await?
            .unwrap();

        let user_relations = page.find_related(user_pages::Entity).all(&ctx.db).await?;
        let mut collaborators = Vec::new();
        for relation in user_relations {
            let user = load_user(&ctx, &relation.user_id).await?;
            collaborators.push(user);
        }

        pages.push(DetailPageResponse::new(&page, collaborators));
    }

    format::json(pages)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("user")
        .add("/current", get(current))
        .add("/current/pages", get(pages))
        .add("/count", get(count))
}
