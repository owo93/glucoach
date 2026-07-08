use axum::{
    Json, RequestPartsExt, extract::FromRequestParts, http::request::Parts, response::IntoResponse,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use glucoach_lib::token::{Claims, TokenError, verify_and_decode};
use reqwest::StatusCode;
use serde_json::json;

#[allow(dead_code)]
pub struct Session(pub(super) Claims);

#[allow(dead_code)]
pub enum AuthError {
    MissingHeader,
    Token(TokenError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            AuthError::MissingHeader => (StatusCode::UNAUTHORIZED, "missing bearer token"),
            AuthError::Token(TokenError::Malformed) => {
                (StatusCode::UNAUTHORIZED, "malformed token")
            }
            AuthError::Token(TokenError::UnknownKey) => {
                (StatusCode::UNAUTHORIZED, "unknown signing key")
            }
            AuthError::Token(TokenError::Expired) => (StatusCode::UNAUTHORIZED, "token expired"),
            AuthError::Token(TokenError::Invalid) => (StatusCode::UNAUTHORIZED, "invalid token"),
        };
        let body = Json(json!({ "error": msg }));
        (status, body).into_response()
    }
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingHeader)?;

        verify_and_decode(bearer.token())
            .map(Session)
            .map_err(AuthError::Token)
    }
}
