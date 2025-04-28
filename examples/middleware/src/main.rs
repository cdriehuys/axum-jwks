use std::net::SocketAddr;

use axum::{middleware, routing::get, Router};
use axum_jwks::Jwks;
use std::env;
use tokio::net::TcpListener;

mod auth;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let jwks = Jwks::from_oidc_url(
        // The Authorization Server that signs the JWTs you want to consume.
        &env::var("AUTHSERVER").unwrap_or(
            "https://my-auth-server.example.com/.well-known/openid-configuration".into(),
        ),
        // The audience identifier for the application. This ensures that
        // JWTs are intended for this application.
        Some(&env::var("AUDIENCE").unwrap_or("https://my-api-identifier.example.com/".into())),
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

    let tcp = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 3000)))
        .await
        .unwrap();
    axum::serve(tcp, router).await.unwrap();
}
