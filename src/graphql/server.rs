use crate::graphql::schema::{MutationRoot, PostgresPool, QueryRoot, SubscriptionRoot};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{QueryBuilder, Schema};
use async_graphql_warp::{graphql_subscription, GQLResponse};
use std::convert::Infallible;
use warp::{http::Response, Filter};

pub async fn start(pool: &PostgresPool) {
  let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
    .data(pool.clone())
    .finish();

  println!("Playground: http://localhost:8000");

  let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
    |(schema, builder): (_, QueryBuilder)| async move {
      let resp = builder.execute(&schema).await;
      Ok::<_, Infallible>(GQLResponse::from(resp))
    },
  );

  let graphql_playground = warp::path::end().and(warp::get()).map(|| {
    Response::builder()
      .header("content-type", "text/html")
      .body(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
      ))
  });

  let routes = graphql_subscription(schema)
    .or(graphql_playground)
    .or(graphql_post);
  warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
