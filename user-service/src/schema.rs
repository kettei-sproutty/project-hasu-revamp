use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub struct Query;

#[Object]
impl Query {
  async fn health_check(&self) -> String {
    "OK".into()
  }
}

pub type UserSchema = Schema<Query, EmptyMutation, EmptySubscription>;
