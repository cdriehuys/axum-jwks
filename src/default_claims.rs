use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{ParseTokenClaims, TokenError};

// The specific claims you want to parse from received JWTs.
#[derive(Deserialize, Serialize)]
pub struct DefaultClaims {}

impl ParseTokenClaims for DefaultClaims {
    type Rejection = TokenClaimsError;
}

pub enum TokenClaimsError {
    Missing,
    Invalid,
}

impl IntoResponse for TokenClaimsError {
    fn into_response(self) -> Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

impl From<TokenError> for TokenClaimsError {
    fn from(value: TokenError) -> Self {
        match value {
            TokenError::Missing => Self::Missing,
            _ => Self::Invalid,
        }
    }
}
