#![doc = include_str!("../README.md")]

//! # Usage
//!
//! Here's a minimal working example of how you would authenticate via JWTs in a
//! route handler:
//! ```no_run
//! use axum::{
//!     async_trait,
//!     extract::{FromRef, FromRequestParts},
//!     http::request::Parts,
//!     http::status::StatusCode,
//!     response::{IntoResponse, Response},
//!     routing::get,
//!     Json,
//!     Router,
//! };
//! use axum_jwks::{Claims, Jwks, ParseTokenClaims, TokenError};
//! use serde::{Deserialize, Serialize};
//!
//! // The state available to all your route handlers.
//! #[derive(Clone)]
//! struct AppState {
//!     jwks: Jwks,
//! }
//!
//! impl FromRef<AppState> for Jwks {
//!     fn from_ref(state: &AppState) -> Self {
//!         state.jwks.clone()
//!     }
//! }
//!
//! // The specific claims you want to parse from received JWTs.
//! #[derive(Deserialize, Serialize)]
//! struct TokenClaims {
//!     pub sub: String
//! }
//!
//! impl ParseTokenClaims for TokenClaims {
//!     type Rejection = TokenClaimsError;
//! }
//!
//! enum TokenClaimsError {
//!     Missing,
//!     Invalid,
//! }
//!
//! impl IntoResponse for TokenClaimsError {
//!     fn into_response(self) -> Response {
//!         // You could do something more informative here like providing a
//!         // response body with different error messages for missing vs.
//!         // invalid tokens.
//!         StatusCode::UNAUTHORIZED.into_response()
//!     }
//! }
//!
//! impl From<TokenError> for TokenClaimsError {
//!     fn from(value: TokenError) -> Self {
//!         match value {
//!             TokenError::Missing => Self::Missing,
//!             other => Self::Invalid,
//!         }
//!     }
//! }
//!
//! // Handler that echos back the claims it receives. If the handler receives
//! // these claims, it's guaranteed that they come from a JWT that is signed
//! // by a key from the JWKS and is valid for the specified audience.
//! async fn echo_claims(Claims(claims): Claims<TokenClaims>) -> Json<TokenClaims> {
//!     Json(claims)
//! }
//!
//! async fn create_router() -> Router<AppState> {
//!     let jwks = Jwks::from_authority(
//!         // The Authorization Server that signs the JWTs you want to consume.
//!         "https://my-auth-server.example.com",
//!         // The audience identifier for the application. This ensures that
//!         // JWTs are intended for this application.
//!         "https://my-api-identifier.example.com/".to_owned(),
//!     )
//!         .await
//!         .unwrap();
//!
//!     Router::new()
//!         .route("/echo-claims", get(echo_claims))
//!         .with_state(AppState { jwks })
//! }
//! ```

mod claims;
mod jwks;
mod token;

pub use claims::{Claims, ParseTokenClaims};
pub use jwks::{JwkError, Jwks, JwksError};
pub use token::{Token, TokenError};
