#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QuerySelect, QueryTrait};
use serde::{Deserialize, Serialize};

use crate::{
    models::_entities::{
        pages::{ActiveModel, Entity, Model},
        user_pages, users,
    },
    views::user::UsersResponse,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: String,
    pub content: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.content = Set(self.content.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn count(State(ctx): State<AppContext>) -> Result<Response> {
    let count = Entity::find().all(&ctx.db).await?.len();
    format::json(count)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let ritem = item.clone();
    item.delete(&ctx.db).await?;
    format::json(ritem)
}

#[debug_handler]
pub async fn get_one(
    auth: auth::ApiToken<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;

    // check if there is a relation between page and current user
    let relation = user_pages::Entity::find()
        .filter(
            model::query::condition()
                .eq(user_pages::Column::UserId, auth.user.id)
                .eq(user_pages::Column::PageId, item.id)
                .build(),
        )
        .one(&ctx.db)
        .await?;

    match relation {
        Some(_) => format::json(load_item(&ctx, id).await?),
        None => Err(Error::Unauthorized("User cannot access this page".into())),
    }
}

/// Return users who can access the page
#[debug_handler]
pub async fn users(
    _auth: auth::ApiToken<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let user_page_relations = item.find_related(user_pages::Entity).all(&ctx.db).await?;

    let mut page_users = Vec::new();
    for user in user_page_relations {
        page_users.push(users::Model::find_by_id(&ctx.db, &user.user_id).await?);
        // TODO : throw custom error when user doesn't exist
    }

    format::json(UsersResponse::from_vec(page_users))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("pages")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
        .add("/:id/users", get(users))
        .add("/count", get(count))
}
