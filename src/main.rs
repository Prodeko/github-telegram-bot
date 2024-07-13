mod api_types;
mod config;
mod github;

use api_types::ApiResult;
use axum::{routing::get, Router};
use config::Config;
use dotenvy::dotenv;
use envconfig::Envconfig;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init_from_env().unwrap();

    serve(&config).await;
}

async fn serve(config: &Config) {
    let app = router();

    axum::serve(
        tokio::net::TcpListener::bind(format!("0.0.0.0:{:?}", config.port))
            .await
            .unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap()
}

fn router() -> Router {
    Router::new()
        .nest("/github", github::router())
        .route("/", get(health))
}

async fn health() -> ApiResult<&'static str> {
    return Ok("ok");
}
