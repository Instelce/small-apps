#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    controllers::user as user_controller,
    errors::ResponseError,
    models::_entities::{
        pages::{self, ActiveModel, Entity, Model},
        user_pages, users,
    },
    views::{
        page::DetailPageResponse,
        user::{UserResponse, UsersResponse},
    },
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
pub async fn list(
    auth: auth::ApiToken<users::Model>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_pages = user_pages::Entity::find()
        .filter(user_pages::Column::UserId.eq(auth.user.id))
        .all(&ctx.db)
        .await?;

    let mut items = Vec::new();
    for rel in user_pages {
        items.push(
            Entity::find()
                .filter(pages::Column::Id.eq(rel.page_id))
                .one(&ctx.db)
                .await?,
        );
    }

    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(
    auth: auth::ApiToken<users::Model>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    // create page
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;

    // create relation
    let relation = user_pages::ActiveModel {
        user_id: Set(auth.user.id),
        page_id: Set(item.id),
        ..Default::default()
    };
    relation.insert(&ctx.db).await?;

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
        Some(_) => {
            let user_relation = item.find_related(user_pages::Entity).all(&ctx.db).await?;
            let mut users = Vec::new();
            for relation in user_relation {
                users.push(user_controller::load_user(&ctx, &relation.user_id).await?);
            }

            format::json(DetailPageResponse::new(&item, users))
        }
        None => Ok(ResponseError::unauthorized("User cannot access this page")),
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
