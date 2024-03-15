use std::{collections::HashMap, str::FromStr};

use jsonwebtoken::{
    jwk::{self, AlgorithmParameters},
    DecodingKey, Validation,
};
use serde::Deserialize;
use thiserror::Error;

use tokio::time::Instant;

const DEFAULT_ALG: jsonwebtoken::Algorithm = jsonwebtoken::Algorithm::RS256;

type Keys = HashMap<String, Jwk>;

#[derive(Clone, Default)]
pub(crate) struct KeyStore {
    pub last_updated: Option<Instant>,
    pub keys: Keys,
}

impl KeyStore {
    pub async fn new(
        client: &reqwest::Client,
        url: &str,
        audience: &str,
    ) -> Result<Self, JwksError> {
        let (jwks_url, alg) = match client.get(url).send().await?.json::<Oidc>().await {
            Ok(oidc) => {
                let jwks_uri = oidc.jwks_uri;
                let alg = match &oidc.id_token_signing_alg_values_supported {
                    Some(algs) => match algs.first() {
                        Some(s) => Some(jsonwebtoken::Algorithm::from_str(s)?),
                        _ => None,
                    },
                    _ => None,
                };
                (jwks_uri.to_string(), alg)
            }
            _ => (url.to_string(), None),
        };
        let keys = Self::from_jwks_url(client, &jwks_url, audience, alg).await?;
        Ok(Self {
            keys,
            last_updated: Some(Instant::now()),
        })
    }

    async fn from_jwks_url(
        client: &reqwest::Client,
        jwks_url: &str,
        audience: &str,
        alg: Option<jsonwebtoken::Algorithm>,
    ) -> Result<Keys, JwksError> {
        let jwks: jwk::JwkSet = client.get(jwks_url).send().await?.json().await?;

        let mut keys = HashMap::new();
        for jwk in jwks.keys {
            let kid = jwk.common.key_id.ok_or(JwkError::MissingKeyId)?;

            match &jwk.algorithm {
                jwk::AlgorithmParameters::RSA(rsa) => {
                    let decoding_key =
                        DecodingKey::from_rsa_components(&rsa.n, &rsa.e).map_err(|err| {
                            JwkError::DecodingError {
                                key_id: kid.clone(),
                                error: err,
                            }
                        })?;
                    let mut validation =
                        Validation::new(jwk.common.algorithm.or(alg).unwrap_or(DEFAULT_ALG));
                    validation.set_audience(&[audience]);

                    keys.insert(
                        kid,
                        Jwk {
                            decoding: decoding_key,
                            validation,
                        },
                    );
                }
                other => {
                    return Err(JwkError::UnexpectedAlgorithm {
                        key_id: kid,
                        algorithm: other.to_owned(),
                    }
                    .into())
                }
            }
        }

        Ok(keys)
    }
}

#[derive(Deserialize)]
struct Oidc {
    jwks_uri: String,
    id_token_signing_alg_values_supported: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct Jwk {
    pub decoding: DecodingKey,
    pub validation: Validation,
}

/// An error with the overall set of JSON Web Keys.
#[derive(Debug, Error)]
pub enum JwksError {
    /// There was an error fetching the OIDC or JWKS config from
    /// the specified authority.
    #[error("could not fetch config from authority: {0}")]
    FetchError(#[from] reqwest::Error),

    /// An error with an individual key caused the processing of the JWKS to
    /// fail.
    #[error("there was an error with an individual key: {0}")]
    KeyError(#[from] JwkError),

    #[error("the provided algorithm from oidc is invalid or empty: {0}")]
    InvalidAlgorithm(#[from] jsonwebtoken::errors::Error),
}

// An error with a specific key from a JWKS.
#[derive(Debug, Error)]
pub enum JwkError {
    /// There was an error constructing the decoding key from the RSA components
    /// provided by the key.
    #[error("could not construct a decoding key for {key_id:?}: {error:?}")]
    DecodingError {
        key_id: String,
        error: jsonwebtoken::errors::Error,
    },

    /// The key is missing the `kid` attribute.
    #[error("the key is missing the `kid` attribute")]
    MissingKeyId,

    /// The key uses an unexpected algorithm type.
    #[error("the key {key_id:?} uses a non-RSA algorithm {algorithm:?}")]
    UnexpectedAlgorithm {
        algorithm: AlgorithmParameters,
        key_id: String,
    },
}
