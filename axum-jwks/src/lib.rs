//! axum-jwks allows for easily verifying JWTs in an axum application using any
//! key from a JSON Web Key Set (JWKS).
//!
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
//! use axum_jwks::{Claims,
//!     KeyManager,
//!     KeyManagerBuilder,
//!     ParseTokenClaims,
//!     TokenError,
//! };
//! use serde::{Deserialize, Serialize};
//! use tokio::time::Duration;
//!
//! // The state available to all your route handlers.
//! #[derive(Clone)]
//! struct AppState {
//!     key_manager: KeyManager,
//! }
//!
//! impl FromRef<AppState> for KeyManager {
//!     fn from_ref(state: &AppState) -> Self {
//!         state.key_manager.clone()
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
//!     let key_manager = KeyManagerBuilder::new(
//!         // The Authorization Server that signs the JWTs you want to consume.
//!         "https://my-auth-server.example.com/.well-known/openid-configuration".to_owned(),
//!         // The audience identifier for the application. This ensures that
//!         // JWTs are intended for this application.
//!         "https://my-api-identifier.example.com/".to_owned(),
//!     ).build().await.unwrap();
//!
//!     Router::new()
//!         .route("/echo-claims", get(echo_claims))
//!         .with_state(AppState { key_manager })
//! }
//! ```

mod claims;
mod key_manager;
mod key_store;
mod token;

pub use claims::{Claims, ParseTokenClaims};
pub use key_manager::{KeyManager, KeyManagerBuilder};
use key_store::KeyStore;
pub use token::{Token, TokenError};
