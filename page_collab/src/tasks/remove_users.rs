//! This task remove all users

use loco_rs::prelude::*;

use crate::models::_entities::users::Entity;

pub struct RemoveUsers;

#[async_trait]
impl Task for RemoveUsers {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "remove_users".into(),
            detail: "Task for removing all users".into(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let users = Entity::find().all(&app_context.db).await?;

        for user in users {
            if user.name != "user1" && user.name != "user2" {
                user.delete(&app_context.db).await?;
            }
        }

        Ok(())
    }
}
