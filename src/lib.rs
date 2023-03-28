use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    RequestPartsExt, TypedHeader,
};

/// A JWT provided as a bearer token in an `Authorization` header.
#[derive(PartialEq)]
pub struct Token(String);

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

#[derive(Debug, PartialEq)]
pub enum TokenError {
    /// No bearer token found in the `Authorization` header.
    Missing,
}

impl IntoResponse for TokenError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::Missing => StatusCode::UNAUTHORIZED,
        };

        status.into_response()
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
