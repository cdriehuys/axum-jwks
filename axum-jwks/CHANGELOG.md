# Changelog

## [0.12.0](https://github.com/cdriehuys/axum-jwks/compare/v0.11.0...v0.12.0) (2025-05-15)


### Features

* do not require jwk.common.key_algorithm ([#52](https://github.com/cdriehuys/axum-jwks/issues/52)) ([7a4c4be](https://github.com/cdriehuys/axum-jwks/commit/7a4c4be6335a5b408d0a6e6317ecdcc8ef362aae))

## [0.11.0](https://github.com/cdriehuys/axum-jwks/compare/v0.10.0...v0.11.0) (2025-01-21)


### Features

* Do not panic for unsupported algorithms ([#48](https://github.com/cdriehuys/axum-jwks/issues/48)) ([8a16978](https://github.com/cdriehuys/axum-jwks/commit/8a1697854b16b3c75a12a31ff667518b4c9bb467))

## [0.10.0](https://github.com/cdriehuys/axum-jwks/compare/v0.9.0...v0.10.0) (2025-01-15)


### ⚠ BREAKING CHANGES

* Drops support for axum versions prior to v0.8

### Features

* Support axum 0.8 ([#46](https://github.com/cdriehuys/axum-jwks/issues/46)) ([c8deed5](https://github.com/cdriehuys/axum-jwks/commit/c8deed57a3e571b556d43b3957fb42bdfa9dc379))

## [0.9.0](https://github.com/cdriehuys/axum-jwks/compare/v0.8.0...v0.9.0) (2025-01-14)


### Features

* Allow constructing a key set from externally fetched data ([#43](https://github.com/cdriehuys/axum-jwks/issues/43)) ([8fa931e](https://github.com/cdriehuys/axum-jwks/commit/8fa931e97df327ed5f60e4a40f7b41369a42809f))

## [0.8.0](https://github.com/cdriehuys/axum-jwks/compare/v0.7.0...v0.8.0) (2024-08-16)


### ⚠ BREAKING CHANGES

* Audience field is optional

### Features

* Audience field is optional ([b4061c1](https://github.com/cdriehuys/axum-jwks/commit/b4061c13d7e7b6922dfccd8f7e43ef51f36ffad7))

## [0.7.0](https://github.com/cdriehuys/axum-jwks/compare/v0.6.2...v0.7.0) (2024-03-14)


### Features

* Allow for switching between native TLS and rustls ([70a100f](https://github.com/cdriehuys/axum-jwks/commit/70a100fb3802618571157ed244e0acdb623db2eb))

## [0.6.2](https://github.com/cdriehuys/axum-jwks/compare/v0.6.1...v0.6.2) (2024-01-29)


### Bug Fixes

* Broken README link ([4a4bd92](https://github.com/cdriehuys/axum-jwks/commit/4a4bd922d67f1a72ccd5cecfdb19c7af86269e0f))

## [0.6.1](https://github.com/cdriehuys/axum-jwks/compare/v0.6.0...v0.6.1) (2024-01-29)


### Bug Fixes

* fix packaging due to sub-directory ([162522a](https://github.com/cdriehuys/axum-jwks/commit/162522ace37852d1f346f1a882d6dcebe0d5f5cf))

## [0.6.0](https://github.com/cdriehuys/axum-jwks/compare/v0.5.0...v0.6.0) (2024-01-29)


### ⚠ BREAKING CHANGES

* support axum 0.7 ([#28](https://github.com/cdriehuys/axum-jwks/issues/28))

### Features

* support axum 0.7 ([#28](https://github.com/cdriehuys/axum-jwks/issues/28)) ([c32943e](https://github.com/cdriehuys/axum-jwks/commit/c32943e9b8418699cf2f09353628b187fabe2dfc))


### Miscellaneous

* Added example for usage in a middleware ([b179b3d](https://github.com/cdriehuys/axum-jwks/commit/b179b3d95973243ed21df941325e732406309cdd))

## [0.5.0](https://github.com/cdriehuys/axum-jwks/compare/v0.4.0...v0.5.0) (2023-08-28)


### ⚠ BREAKING CHANGES

* Use oidc to get jwks etc
* Require entire jwks url

### Features

* Require entire jwks url ([f9f02a8](https://github.com/cdriehuys/axum-jwks/commit/f9f02a8897c5d806b105ceed23a317401f1d35ab))
* Use oidc to get jwks etc ([330a050](https://github.com/cdriehuys/axum-jwks/commit/330a050abf1ce42bf08edf74a97f529fcb259320))


### Miscellaneous

* Update README.md ([38a4d9c](https://github.com/cdriehuys/axum-jwks/commit/38a4d9c6461efa186a4654da49cf916e118910eb))

## [0.4.0](https://github.com/cdriehuys/axum-jwks/compare/v0.3.0...v0.4.0) (2023-04-05)


### Features

* Add trait to handle `FromRequestParts` ([44997aa](https://github.com/cdriehuys/axum-jwks/commit/44997aae1e492ad25bb1b488aa55b783a6e847a9))


### Documentation

* Add usage example ([23800eb](https://github.com/cdriehuys/axum-jwks/commit/23800eb658f8ab5aa82e9558a28312ee19f00687))


### Miscellaneous

* Add debug tracing for token validation ([b345f5d](https://github.com/cdriehuys/axum-jwks/commit/b345f5da4f954c8feb92cc9771c5556ac03ff697))

## [0.3.0](https://github.com/cdriehuys/axum-jwks/compare/v0.2.0...v0.3.0) (2023-03-30)


### ⚠ BREAKING CHANGES

* Validate audience claim from tokens.

### Features

* Validate audience claim from tokens. ([4cb607f](https://github.com/cdriehuys/axum-jwks/commit/4cb607f1792dd4b94571a48a04d2572155df3697))


### Miscellaneous Chores

* Release 0.3.0 ([aefbfa6](https://github.com/cdriehuys/axum-jwks/commit/aefbfa6526e1bd891fd329d49831e84c7b8c4944))
* Release 0.3.0 ([b4bd52f](https://github.com/cdriehuys/axum-jwks/commit/b4bd52f6f49ef96cbf2967c89e75ec05e4a08086))

## [0.2.0](https://github.com/cdriehuys/axum-jwks/compare/v0.1.2...v0.2.0) (2023-03-29)


### ⚠ BREAKING CHANGES

* Rename to `axum-jwks`.

### Miscellaneous Chores

* Override release version. ([357f1cc](https://github.com/cdriehuys/axum-jwks/commit/357f1cc3b8d43dcb7634c236de4eba35aa2cbeef))
* Rename to `axum-jwks`. ([72be46a](https://github.com/cdriehuys/axum-jwks/commit/72be46ab34ef75e244d4224794f536de79f3e6c6))

## [0.1.2](https://github.com/cdriehuys/auth0-axum/compare/v0.1.1...v0.1.2) (2023-03-29)


### Bug Fixes

* Add required packaging information. ([3cd50c5](https://github.com/cdriehuys/auth0-axum/commit/3cd50c52b263caa7215e6031924d9a3531ba3030))

## [0.1.1](https://github.com/cdriehuys/auth0-axum/compare/v0.1.0...v0.1.1) (2023-03-29)


### Bug Fixes

* Fix typo in publish logic. ([f1804b3](https://github.com/cdriehuys/auth0-axum/commit/f1804b31cfcb4ae587f2be000d24fb099efe4930))

## 0.1.0 (2023-03-29)


### Features

* Access token value. ([d390d48](https://github.com/cdriehuys/auth0-axum/commit/d390d4866b02bcac448bbab19cb8199b6c23f95a))
* Add `Token` type. ([04a6a6c](https://github.com/cdriehuys/auth0-axum/commit/04a6a6c12e40b022892dea2b2e63328785d7c7e6))
* Add JWKS container. ([6567b61](https://github.com/cdriehuys/auth0-axum/commit/6567b6153430a371da9db5c4e3d6be213fa98278))


### Bug Fixes

* Make `Jwks` cloneable. ([fff1684](https://github.com/cdriehuys/auth0-axum/commit/fff16842afac25861853c7802485fde1e2038334))
