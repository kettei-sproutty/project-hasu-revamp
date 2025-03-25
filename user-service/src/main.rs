use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, ID, Object, Schema, SimpleObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};

#[derive(SimpleObject)]
struct Profile {
  id: ID,
}

struct Query;

#[Object]
impl Query {
  async fn me(&self) -> Profile {
    Profile {
      id: ID("123".into()),
    }
  }
}

type UserSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[tokio::main]
async fn main() {
  let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
    .enable_federation()
    .finish();

  let app = Router::new()
    .route("/", get(graphiql).post(graphql_handler))
    .layer(Extension(schema));

  let listener = tokio::net::TcpListener::bind("127.0.0.1:4001")
    .await
    .unwrap();

  axum::serve(listener, app).await.unwrap();
}

async fn graphql_handler(schema: Extension<UserSchema>, req: GraphQLRequest) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
  axum::response::Html(GraphiQLSource::build().endpoint("/").finish())
}
