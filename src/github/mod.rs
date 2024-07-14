use axum::{debug_handler, extract::State, routing::post, Json, Router};
mod format_push_event;
use format_push_event::format_push_event;
use types::PushEvent;
pub mod types;
use crate::{
    api_types::{ApiError, ApiResult},
    config::Settings,
    telegram::send_message,
    AppState,
};
pub fn router() -> Router<AppState> {
    Router::new().route("/push", post(post_push))
}

#[debug_handler]
async fn post_push(state: State<AppState>, Json(event): Json<PushEvent>) -> ApiResult<()> {
    let res = send_message(
        &format_push_event(&event),
        state.config.chat_id,
        state.config.authorized_telegram_url(),
    )
    .await;

    return res.map(|r| println!("{:?}", r)).map_err(|e| match e {
        reqwest_middleware::Error::Middleware(e) => ApiError::InternalServerError(e.to_string()),
        reqwest_middleware::Error::Reqwest(e) => {
            if e.is_timeout() {
                ApiError::GatewayTimeout(e.to_string())
            } else {
                ApiError::InternalServerError(e.to_string())
            }
        }
    });
}
