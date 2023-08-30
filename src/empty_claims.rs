use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{ParseTokenClaims, TokenError};

/// Use this when only verifying the audience claim,
/// and the endpoint do not need anything from the token
#[derive(Deserialize, Serialize)]
pub struct EmptyClaims {}

impl ParseTokenClaims for EmptyClaims {
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
