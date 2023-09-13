use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::IntoResponse,
};
use serde::de::DeserializeOwned;

use crate::{KeyManager, Token, TokenError};

pub struct Claims<C: DeserializeOwned + ParseTokenClaims>(pub C);

/// Trait indicating that the type can be parsed from a request.
///
/// Implementing this trait for your claims data means it can be used as an
/// [extractor][axum::extract] in your request handlers. Assuming you have a
/// struct `TokenClaims` with the attributes you want to parse from a JWT,
/// implementing [`ParseTokenClaims`] will allow you to write a request handler
/// like:
/// ```ignore
/// async fn my_request_handler(Claims(claims): Claims<TokenClaims>) -> Response {
///     todo!()
/// }
/// ```
///
/// The alternative to implementing this trait is to implement
/// [`FromRequestParts`][axum::extract::FromRequestParts] directly for your
/// token claims.
///
/// # Example
/// ```
/// use axum::{
///     http::status::StatusCode,
///     response::{IntoResponse, Response},
///     Json,
/// };
/// use axum_jwks::{ParseTokenClaims, TokenError};
/// use serde::Deserialize;
/// use serde_json::json;
///
/// #[derive(Deserialize)]
/// struct TokenClaims {
///     sub: String,
/// }
///
/// impl ParseTokenClaims for TokenClaims {
///     type Rejection = TokenClaimsError;
/// }
///
/// enum TokenClaimsError {
///     Invalid,
///     Missing,
/// }
///
/// impl From<TokenError> for TokenClaimsError {
///     fn from(error: TokenError) -> Self {
///         match error {
///             TokenError::Missing => Self::Missing,
///             other => Self::Invalid,
///         }
///     }
/// }
///
/// impl IntoResponse for TokenClaimsError {
///     fn into_response(self) -> Response {
///         let body = match self {
///             Self::Invalid => json!({ "message": "Invalid token." }),
///             Self::Missing => json!({ "message": "No token provided." }),
///         };
///
///         (StatusCode::UNAUTHORIZED, Json(body)).into_response()
///     }
/// }
/// ```
pub trait ParseTokenClaims {
    /// The type of error returned if the token claims cannot be parsed and
    /// validated from the request.
    type Rejection: IntoResponse + From<TokenError>;
}

#[async_trait]
impl<S, C> FromRequestParts<S> for Claims<C>
where
    C: DeserializeOwned + ParseTokenClaims,
    KeyManager: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = C::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let key_manager = KeyManager::from_ref(state);
        let token = Token::from_request_parts(parts, state).await?;

        let token_data = key_manager.validate_claims(token.value()).await?;

        Ok(Claims(token_data.claims))
    }
}
