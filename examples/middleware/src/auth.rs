use axum::{
    extract::{FromRef, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use axum_jwks::KeyManager;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Claims {}

#[derive(Clone)]
pub struct AppState {
    pub key_manager: KeyManager,
}

impl FromRef<AppState> for KeyManager {
    fn from_ref(state: &AppState) -> Self {
        state.key_manager.clone()
    }
}

/// A simple middleware that only verifies the default claims: `aud`, `exp`
/// axum-jwks by default verifies `aud`
/// jsonwebtoken by default validates `exp`
///
/// This kind of middleware can be used to simply protect the endpoints
/// but there is no need to use content from the claims in the token.
///
/// If any content of the claims are required in the endpoints,
/// please follow the example in lib.rs.
///
/// Add fields to the `Claims` struct if more claims are required to be present.
/// Add code in this function to do any validation of their vaules.
///
pub async fn validate_token(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    request: Request,
    next: Next,
) -> Response {
    let jwks = KeyManager::from_ref(&state);

    if jwks
        .validate_claims::<Claims>(bearer.token())
        .await
        .is_err()
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(request).await
}
