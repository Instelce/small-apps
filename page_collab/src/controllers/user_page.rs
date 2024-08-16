#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{
    _entities::user_pages::{ActiveModel, Entity, Model},
    users::users,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    user_id: i32,
    page_id: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.user_id = Set(self.user_id.clone());
        item.page_id = Set(self.page_id.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
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

pub async fn remove(
    _auth: auth::ApiToken<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let ritem = item.clone();
    item.delete(&ctx.db).await?;
    format::json(ritem)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("user_pages")
        .add("/", post(add))
        .add("/:id", delete(remove))
}
