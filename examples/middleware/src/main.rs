use std::net::SocketAddr;

use axum::{middleware, routing::get, Router};
use axum_jwks::Jwks;

mod auth;

#[tokio::main]
async fn main() {
    let jwks = Jwks::from_oidc_url(
        // The Authorization Server that signs the JWTs you want to consume.
        "https://my-auth-server.example.com/.well-known/openid-configuration",
        // The audience identifier for the application. This ensures that
        // JWTs are intended for this application.
        "https://my-api-identifier.example.com/".to_owned(),
    )
    .await
    .unwrap();

    let state = auth::AppState { jwks };
    let router = Router::new()
        .route("/", get(|| async { "ok" }))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::validate_token,
        ))
        .with_state(state);

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(router.into_make_service())
        .await
        .unwrap();
}
