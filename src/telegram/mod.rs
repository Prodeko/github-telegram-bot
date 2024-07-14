mod types;
use reqwest::Url;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use types::SendMessageRequest;

pub async fn send_message<'a, 'b>(
    message: &'a str,
    chat_id: i64,
    url: Url,
) -> Result<reqwest::Response, reqwest_middleware::Error> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let message_body = SendMessageRequest {
        chat_id: chat_id,
        text: message.to_owned(),
        parse_mode: Some("Markdown".to_string()),
        ..Default::default()
    };

    let res = client
        .post(url.join("sendMessage").unwrap())
        .json(&message_body)
        .send()
        .await;

    return res;
}
