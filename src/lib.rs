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
//!     response::IntoResponse,
//!     routing::get,
//!     Json,
//!     Router,
//! };
//! use serde::{Deserialize, Serialize};
//!
//! // The state available to all your route handlers.
//! #[derive(Clone)]
//! struct AppState {
//!     jwks: axum_jwks::Jwks,
//! }
//!
//! // The specific claims you want to parse from received JWTs.
//! #[derive(Deserialize, Serialize)]
//! struct TokenClaims {
//!     pub sub: String
//! }
//!
//! #[async_trait]
//! impl<S> FromRequestParts<S> for TokenClaims
//! where
//!     AppState: FromRef<S>,
//!     S: Send + Sync
//! {
//!     // If you provide a custom type here, you can implement `IntoResponse`
//!     // for it to provide the exact error messages you need.
//!     type Rejection = axum_jwks::TokenError;
//!
//!     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//!         let jwks = AppState::from_ref(state).jwks;
//!         let token = axum_jwks::Token::from_request_parts(parts, state).await?;
//!
//!         let token_data = jwks.validate_claims(token.value())?;
//!
//!         Ok(token_data.claims)
//!     }
//! }
//!
//! // Handler that echos back the claims it receives. If the handler receives
//! // these claims, it's guaranteed that they come from a JWT that is signed
//! // by a key from the JWKS and is valid for the specified audience.
//! async fn echo_claims(claims: TokenClaims) -> Json<TokenClaims> {
//!     Json(claims)
//! }
//!
//! async fn create_router() -> Router<AppState> {
//!     let jwks = axum_jwks::Jwks::from_authority(
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

mod jwks;
mod token;

pub use jwks::{JwkError, Jwks, JwksError};
pub use token::{Token, TokenError};
