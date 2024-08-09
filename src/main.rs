mod api_types;
mod config;
mod github;
mod telegram;

use std::sync::Arc;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

use api_types::ApiResult;
use axum::{debug_handler, routing::get, Router};
use config::{Config, Settings};
use dotenvy::dotenv;
use envconfig::Envconfig;
use telegram::send_message;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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
    let chat_id = config.chat_id.clone();
    let telegram_url = config.authorized_telegram_url();

    let app = router(config);

    send_message("Webbitiimibot running!", chat_id, telegram_url.clone())
        .await
        .unwrap();

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
        .nest("/github", github::router(state.clone()))
        .route("/", get(health))
        .with_state(state);

    return r;
}

#[debug_handler]
async fn health() -> ApiResult<&'static str> {
    return Ok("ok");
}
