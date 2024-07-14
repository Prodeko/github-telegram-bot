mod api_types;
mod config;
mod github;
mod telegram;

use std::sync::Arc;

use api_types::ApiResult;
use axum::{debug_handler, extract::State, routing::get, Router};
use config::Config;
use dotenvy::dotenv;
use envconfig::Envconfig;

#[derive(Clone)]
struct AppState {
    config: Arc<Config>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init_from_env().unwrap();

    serve(config).await;
}

async fn serve(config: Config) {
    let port = config.port.clone();
    let app = router(config);

    axum::serve(
        tokio::net::TcpListener::bind(format!("0.0.0.0:{:?}", port))
            .await
            .unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap()
}

fn router(state: Config) -> Router<()> {
    let state: AppState = AppState {
        config: Arc::new(state),
    };

    let r = Router::new()
        .nest("/github", github::router())
        .route("/", get(health))
        .with_state(state);

    return r;
}

#[debug_handler]
async fn health(state: State<AppState>) -> ApiResult<&'static str> {
    return Ok("ok");
}
