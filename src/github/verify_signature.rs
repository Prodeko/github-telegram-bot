use axum::{
    body::{self, Body},
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::{
    api_types::{ApiError, ApiResult, AuthenticationError},
    AppState,
};

const GITHUB_AUTHORIZATION_HEADER: &str = "X-Hub-Signature-256";

type HmacSha256 = Hmac<Sha256>;

/// Verification of webhook payload and authenticity using HMAC and SHA256.
/// See [Github docs](https://docs.github.com/en/webhooks/using-webhooks/validating-webhook-deliveries#examples) for further reference.
pub fn verify_signature<'a, 'b, 'c>(
    auth_header: &'a str,
    body: &'b [u8],
    token: &'c str,
) -> Result<(), ()> {
    let mut mac =
        HmacSha256::new_from_slice(token.as_bytes()).expect("HMAC can take key of any size");

    let trimmed_auth_header = auth_header.strip_prefix("sha256=").ok_or(())?;

    let auth_header_hex_bytes = hex::decode(trimmed_auth_header).map_err(|e| println!("{e}"))?;

    mac.update(body);

    return mac
        .verify_slice(&auth_header_hex_bytes)
        .map_err(|e| println!("{e}"));
}

pub async fn validate_github_webhook(
    headers: HeaderMap,
    state: State<AppState>,
    req: Request,
    next: Next,
) -> ApiResult<Response> {
    let auth_header = headers
        .get(GITHUB_AUTHORIZATION_HEADER)
        .map(|h| h.to_str())
        .ok_or(ApiError::AuthenticationError(
            crate::api_types::AuthenticationError::HeaderNotFound,
        ))?
        .map_err(|_| ApiError::AuthenticationError(AuthenticationError::InvalidToken))?;

    let (parts, body) = req.into_parts();

    let bytes = body::to_bytes(body, 2_000_000)
        .await
        .map_err(|_| ApiError::BadRequest)?;

    verify_signature(auth_header, &bytes, &state.config.github_token)
        .map_err(|_| ApiError::AuthenticationError(AuthenticationError::InvalidToken))?;

    let new_body: Body = bytes.into();

    let request = Request::from_parts(parts, new_body);

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use crate::github::verify_signature::verify_signature;

    #[test]
    fn test_signature_verification_against_github_example() {
        verify_signature(
            "sha256=757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17",
            &"Hello, World!".as_bytes(),
            "It's a Secret to Everybody",
        )
        .expect("Validation against example data provided by Github failed");
    }

    #[test]
    fn test_signature_verification_against_altered_github_example() {
        verify_signature(
            "sha256=057107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17",
            &"Hello, World!".as_bytes(),
            "It's a Secret to Everybody",
        )
        .expect_err("Expected validation to fail when auth header was modified but it succeeded");
    }
}
