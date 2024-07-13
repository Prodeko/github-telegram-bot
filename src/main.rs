mod api_types;
mod config;
mod routes;

use axum::Router;
use config::Config;
use dotenvy::dotenv;
use envconfig::Envconfig;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init_from_env().unwrap();

    serve(&config).await;
}

async fn serve(config: &Config) {
    let app = router();

    axum::Server::bind(&format!("0.0.0.0:{:?}", config.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new().nest("/slack", routes::slack::router())
}
