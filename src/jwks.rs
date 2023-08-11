use std::collections::HashMap;

use jsonwebtoken::{
    decode, decode_header,
    jwk::{self, AlgorithmParameters},
    DecodingKey, TokenData, Validation,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tracing::{debug, info};

use crate::TokenError;

/// A container for a set of JWT decoding keys.
///
/// The container can be used to validate any JWT that identifies a known key
/// through the `kid` attribute in the token's header.
#[derive(Clone)]
pub struct Jwks {
    keys: HashMap<String, Jwk>,
}

impl Jwks {
    /// Pull a JSON Web Key Set from a specific authority.
    ///
    /// # Arguments
    /// * `jwks_url` - The url which JWKS info is pulled from.
    /// * `audience` - The identifier of the consumer of the JWT. This will be
    ///   matched against the `aud` claim from the token.
    ///
    /// # Return Value
    /// The information needed to decode JWTs using any of the keys specified in
    /// the authority's JWKS.
    pub async fn from_jwks_url(jwks_url: &str, audience: String) -> Result<Self, JwksError> {
        Self::from_jwks_url_with_client(&reqwest::Client::default(), jwks_url, audience).await
    }

    /// A version of [`from_jwks`][Self::from_jwks] that allows for
    /// passing in a custom [`Client`][reqwest::Client].
    pub async fn from_jwks_url_with_client(
        client: &reqwest::Client,
        jwks_url: &str,
        audience: String,
    ) -> Result<Self, JwksError> {
        debug!(%jwks_url, "Fetching JSON Web Key Set.");
        let jwks: jwk::JwkSet = client.get(jwks_url).send().await?.json().await?;
        info!(
            %jwks_url,
            count = jwks.keys.len(),
            "Successfully pulled JSON Web Key Set."
        );

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
                    let mut validation = Validation::new(jwk.common.algorithm.ok_or(
                        JwkError::MissingAlgorithm {
                            key_id: kid.clone(),
                        },
                    )?);
                    validation.set_audience(&[audience.clone()]);

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

        Ok(Self { keys })
    }

    pub fn validate_claims<T>(&self, token: &str) -> Result<TokenData<T>, TokenError>
    where
        T: DeserializeOwned,
    {
        let header = decode_header(token).map_err(|error| {
            debug!(?error, "Received token with invalid header.");

            TokenError::InvalidHeader(error)
        })?;
        let kid = header.kid.as_ref().ok_or_else(|| {
            debug!(?header, "Header is missing the `kid` attribute.");

            TokenError::MissingKeyId
        })?;

        let key = self.keys.get(kid).ok_or_else(|| {
            debug!(%kid, "Token refers to an unknown key.");

            TokenError::UnknownKeyId(kid.to_owned())
        })?;

        let decoded_token: TokenData<T> =
            decode(token, &key.decoding, &key.validation).map_err(|error| {
                debug!(?error, "Token is malformed or does not pass validation.");

                TokenError::Invalid(error)
            })?;

        Ok(decoded_token)
    }
}

#[derive(Clone)]
struct Jwk {
    decoding: DecodingKey,
    validation: Validation,
}

/// An error with the overall set of JSON Web Keys.
#[derive(Debug, Error)]
pub enum JwksError {
    /// There was an error fetching the JWKS from the specified authority.
    #[error("could not fetch JWKS from authority: {0}")]
    FetchError(#[from] reqwest::Error),

    /// An error with an individual key caused the processing of the JWKS to
    /// fail.
    #[error("there was an error with an individual key: {0}")]
    KeyError(#[from] JwkError),
}

/// An error with a specific key from a JWKS.
#[derive(Debug, Error)]
pub enum JwkError {
    /// There was an error constructing the decoding key from the RSA components
    /// provided by the key.
    #[error("could not construct a decoding key for {key_id:?}: {error:?}")]
    DecodingError {
        key_id: String,
        error: jsonwebtoken::errors::Error,
    },

    /// The key does not specify an algorithm to use.
    #[error("the key {key_id:?} does not specify an algorithm")]
    MissingAlgorithm { key_id: String },

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
