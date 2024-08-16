#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use page_collab::{app::App, models::_entities::user_pages};

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    // let active_model: articles::ActiveModel = ActiveModel {
    //     title: Set(Some("how to build apps in 3 steps".to_string())),
    //     content: Set(Some("use Loco: https://loco.rs".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);

    let user_page_relations = user_pages::Entity::find().all(&ctx.db).await.unwrap();
    for user in user_page_relations {
        user.into_active_model().delete(&ctx.db).await.unwrap();
    }

    Ok(())
}
