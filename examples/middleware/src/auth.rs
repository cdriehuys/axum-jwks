use axum::{
    extract::{FromRef, State},
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    TypedHeader,
};
use axum_jwks::Jwks;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Claims {}

#[derive(Clone)]
pub struct AppState {
    pub jwks: Jwks,
}

impl FromRef<AppState> for Jwks {
    fn from_ref(state: &AppState) -> Self {
        state.jwks.clone()
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
pub async fn validate_token<B>(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let jwks = Jwks::from_ref(&state);

    if jwks.validate_claims::<Claims>(bearer.token()).is_err() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(request).await
}
