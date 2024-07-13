use axum::{debug_handler, routing::post, Json, Router};
mod format_push_event;
use format_push_event::format_push_event;
use types::PushEvent;
pub mod types;
use crate::api_types::ApiResult;

pub fn router() -> Router {
    Router::new().route("/push", post(post_push))
}

#[debug_handler]
async fn post_push(Json(event): Json<PushEvent>) -> ApiResult<String> {
    return Ok(format_push_event(&event));
}
