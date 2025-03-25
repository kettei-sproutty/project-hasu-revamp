use async_graphql::SimpleObject;
use sqlx::prelude::FromRow;

#[derive(FromRow, SimpleObject)]
pub struct Profile {
  pub id: uuid::Uuid,
  pub user_id: uuid::Uuid,
  pub username: String,
  pub avatar_url: Option<String>,
  pub biography: Option<String>,
}
