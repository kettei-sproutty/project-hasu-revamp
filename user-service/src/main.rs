use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Extension, Router, response::IntoResponse, routing::get};

mod schema;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let port = std::env::var("PORT").unwrap_or_else(|_| "4001".to_string());

  let schema = schema::UserSchema::build(
    schema::Query,
    async_graphql::EmptyMutation,
    async_graphql::EmptySubscription,
  )
  .enable_federation()
  .finish();

  let app = Router::new()
    .route("/", get(graphiql).post(graphql_handler))
    .layer(Extension(schema));

  let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
    .await
    .unwrap();

  axum::serve(listener, app).await.unwrap();
}

async fn graphql_handler(
  schema: Extension<schema::UserSchema>,
  req: GraphQLRequest,
) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
  axum::response::Html(GraphiQLSource::build().endpoint("/").finish())
}
