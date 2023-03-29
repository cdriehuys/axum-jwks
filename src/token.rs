use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    RequestPartsExt, TypedHeader,
};
use thiserror::Error;

/// A JWT provided as a bearer token in an `Authorization` header.
#[derive(PartialEq)]
pub struct Token(String);

impl Token {
    /// Get the token's value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Standard debug format but with literal value redacted.
        f.debug_tuple("Token").field(&"*".repeat(8)).finish()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Token {
    type Rejection = TokenError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| TokenError::Missing)?;

        Ok(Self(bearer.token().to_owned()))
    }
}

/// An error with a JWT.
#[derive(Debug, Error, PartialEq)]
pub enum TokenError {
    /// The token is either malformed or did not pass validation.
    #[error("the token is invalid or malformed: {0:?}")]
    Invalid(jsonwebtoken::errors::Error),

    /// The token header could not be decoded because it was malformed.
    #[error("the token header is malformed: {0:?}")]
    InvalidHeader(jsonwebtoken::errors::Error),

    /// No bearer token found in the `Authorization` header.
    #[error("no bearer token found")]
    Missing,

    /// The token's header does not contain the `kid` attribute used to identify
    /// which decoding key should be used.
    #[error("the token header does not specify a `kid`")]
    MissingKeyId,

    /// The token's `kid` attribute specifies a key that is unknown.
    #[error("token uses the unknown key {0:?}")]
    UnknownKeyId(String),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::http::{header::AUTHORIZATION, Request};

    use super::*;

    #[test]
    fn debug_does_not_contain_value() {
        let token_value = "some-secret-value";
        let token = Token(token_value.to_owned());
        let debug = format!("{:?}", token);

        assert!(
            !debug.contains(token_value),
            "The debug representation of the token {:?} should not contain the raw token value {:?}",
            debug,
            token_value,
        );
    }

    #[tokio::test]
    async fn parse_from_header() {
        let token = "some-token-value";
        let request = Request::builder()
            .uri("https://example.com")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(())
            .unwrap();

        let (mut parts, _) = request.into_parts();

        let parsed = Token::from_request_parts(&mut parts, &()).await.unwrap();

        assert_eq!(Token(token.to_owned()), parsed);
    }

    #[tokio::test]
    async fn parse_from_header_missing_authorization() {
        let request = Request::builder()
            .uri("https://example.com")
            .body(())
            .unwrap();

        let (mut parts, _) = request.into_parts();

        let err = Token::from_request_parts(&mut parts, &())
            .await
            .expect_err("Missing `Authorization` header should cause an error.");

        assert_eq!(TokenError::Missing, err);
    }
}
