use async_graphql::http::GraphiQLSource;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use std::net::SocketAddr;
use tracing::info;

mod models;
mod mutation;
mod query;
mod schema;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    info!("async-graphql start");

    let service = schema::service();

    let app = Router::new().route("/", get(graphiql).post_service(service));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("GraphiQL: {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
