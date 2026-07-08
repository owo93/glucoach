use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::jwks::SUPABASE_JWKS;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: Uuid,
    pub aud: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub iat: DateTime<Utc>,
    pub session_id: Uuid,
    pub email: String,
}

#[derive(Debug)]
pub enum TokenError {
    Malformed,
    UnknownKey,
    Expired,
    Invalid,
}

pub fn verify_and_decode(token: &str) -> Result<Claims, TokenError> {
    let header = decode_header(token).map_err(|_| TokenError::Malformed)?;
    let kid = header.kid.ok_or(TokenError::Malformed)?;

    let jwks = SUPABASE_JWKS.load();
    let jwk = jwks.find(&kid).ok_or(TokenError::UnknownKey)?;
    let decoding_key = DecodingKey::from_jwk(jwk).map_err(|_| TokenError::Invalid)?;
    drop(jwks);

    let mut validation = Validation::new(jsonwebtoken::Algorithm::ES256);
    validation.validate_aud = false;
    validation.set_required_spec_claims(&["exp", "iss", "sub"]);

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| {
            tracing::warn!(error = ?e, kind = ?e.kind(), "JWT decode/validation failed");
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => TokenError::Expired,
                _ => TokenError::Invalid,
            }
        })
}
