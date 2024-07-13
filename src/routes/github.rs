use axum::{debug_handler, response::Html, routing::get, Router};

use crate::api_types::ApiResult;

pub fn router() -> Router {
    Router::new().route("/hello", get(get_hello))
}

#[debug_handler]
async fn get_hello() -> ApiResult<Html<String>> {
    return Ok(Html("Hello world from slack.rs".to_string()));
}
