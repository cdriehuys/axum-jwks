# axum-jwks

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/cdriehuys/axum-jwks/rust.yml?branch=main)](https://github.com/cdriehuys/axum-jwks/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/axum-jwks)](https://crates.io/crates/axum-jwks)
[![docs.rs](https://img.shields.io/docsrs/axum-jwks)](https://docs.rs/axum-jwks)

Use a [JSON Web Key Set (JWKS)][jwks] to verify JWTs in [Axum][axum].

# Features

* Pull a JWKS from an Authorization Server
* Verify JWTs signed by any key in the JWKS and provided as a bearer token in
  the `Authorization` header

For more information, see the [crate documentation][axum-jwks-docs].

[axum]: https://github.com/tokio-rs/axum
[axum-jwks-docs]: https://docs.rs/axum-jwks
[jwks]: https://auth0.com/docs/secure/tokens/json-web-tokens/json-web-key-sets
