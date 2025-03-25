use async_graphql::EmptySubscription;
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use sqlx::PgPool;

mod db;
mod models;
mod schema;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

  let port = std::env::var("PORT").expect("PORT must be set in .env");

  let pool = PgPool::connect(&database_url)
    .await
    .expect("Cannot connect to DB");

  let db = db::DB { pool };

  let schema = schema::ProfileSchema::build(schema::Query, schema::Mutation, EmptySubscription)
    .enable_federation()
    .data(db)
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
  schema: Extension<schema::ProfileSchema>,
  req: GraphQLRequest,
) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
  axum::response::Html(GraphiQLSource::build().endpoint("/").finish())
}
