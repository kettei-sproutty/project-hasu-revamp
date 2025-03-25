use async_graphql::{Context, EmptySubscription, ID, Object, Result, Schema};
use uuid::Uuid;

use crate::{db::DB, models::Profile};

pub struct Query;

#[Object]
impl Query {
  async fn me(&self) -> Result<Option<Profile>> {
    Ok(None)
  }

  async fn get_profile_by_user_id(
    &self,
    context: &Context<'_>,
    user_id: ID,
  ) -> Result<Option<Profile>> {
    let db = context.data::<DB>()?;
    let uuid = Uuid::parse_str(&user_id)?;
    let profile = db.get_profile_by_user_id(uuid).await?;

    Ok(profile)
  }

  async fn get_all_profiles(&self, context: &Context<'_>) -> Result<Vec<Profile>> {
    let db = context.data::<DB>()?;

    let profiles = db.get_profiles().await?;

    Ok(profiles)
  }
}

pub struct Mutation;

#[Object]
impl Mutation {
  async fn update_profile(&self) -> Result<Option<Profile>> {
    Ok(None)
  }
}

pub type ProfileSchema = Schema<Query, Mutation, EmptySubscription>;
