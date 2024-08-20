use sea_orm::DeriveEntityModel;

pub mod auth;
pub mod user;

pub trait FromModel<T> {
    fn new(model: &T) -> Self;
}
