use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(UserPages::Table)
                    .col(pk_auto(UserPages::Id))
                    .col(integer(UserPages::UserId))
                    .col(integer(UserPages::PageId))
                    .index(
                        Index::create()
                            .name("idx-user-page")
                            .unique()
                            .col(UserPages::UserId)
                            .col(UserPages::PageId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_pages-users")
                            .from(UserPages::Table, UserPages::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_pages-pages")
                            .from(UserPages::Table, UserPages::PageId)
                            .to(Pages::Table, Pages::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserPages {
    Table,
    Id,
    UserId,
    PageId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Pages {
    Table,
    Id,
}
