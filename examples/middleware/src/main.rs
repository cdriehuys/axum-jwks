use std::net::SocketAddr;

use axum::{middleware, routing::get, Router};
use axum_jwks::KeyManagerBuilder;
use reqwest::Client;
use std::env;
use tokio::net::TcpListener;

mod auth;

#[tokio::main]
async fn main() {
    let key_manager = KeyManagerBuilder::new(
        // The Authorization Server that signs the JWTs you want to consume.
        env::var("AUTHSERVER").expect("$AUTHSERVER env variable"),
        // The audience identifier for the application. This ensures that
        // JWTs are intended for this application.
        env::var("AUDIENCE").expect("$AUDIENCE env variable"),
    )
    .minimal_update_interval(600)
    .client(Client::default())
    .build()
    .await
    .unwrap();

    let state = auth::AppState { key_manager };
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
