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
pub struct Jwks {
    keys: HashMap<String, Jwk>,
}

impl Jwks {
    /// Pull a JSON Web Key Set from a specific authority.
    ///
    /// # Arguments
    /// * `authority` - The base domain that will be issuing keys. The JWKS info
    ///   is pulled from `{authority}/.well-known/jwks.json`.
    ///
    /// # Return Value
    /// The information needed to decode JWTs using any of the keys specified in
    /// the authority's JWKS.
    pub async fn from_authority(authority: &str) -> Result<Self, JwksError> {
        Self::from_authority_with_client(&reqwest::Client::default(), authority).await
    }

    /// A version of [`from_authority`][Self::from_authority] that allows for passing in a custom
    /// [`Client`][reqwest::Client].
    pub async fn from_authority_with_client(
        client: &reqwest::Client,
        authority: &str,
    ) -> Result<Self, JwksError> {
        let jwks_url = format!("{}/.well-known/jwks.json", authority);
        debug!(%authority, %jwks_url, "Fetching JSON Web Key Set.");
        let jwks: jwk::JwkSet = client.get(jwks_url).send().await?.json().await?;

        info!(
            %authority,
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
                    let validation = Validation::new(jwk.common.algorithm.ok_or(
                        JwkError::MissingAlgorithm {
                            key_id: kid.clone(),
                        },
                    )?);

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
        let header = decode_header(token).map_err(TokenError::InvalidHeader)?;
        let kid = header.kid.ok_or(TokenError::MissingKeyId)?;

        let key = self
            .keys
            .get(&kid)
            .ok_or_else(|| TokenError::UnknownKeyId(kid))?;

        let decoded_token: TokenData<T> =
            decode(token, &key.decoding, &key.validation).map_err(TokenError::Invalid)?;

        Ok(decoded_token)
    }
}

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
