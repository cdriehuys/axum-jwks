use std::sync::Arc;

use crate::key_store::JwksError;
use crate::{KeyStore, TokenError};
use jsonwebtoken::{decode, decode_header, TokenData};
use serde::de::DeserializeOwned;

use tokio::sync::RwLock;

use tokio::time::{Duration, Instant};
use tracing::{debug, info};

#[derive(Clone)]
pub struct KeyManager {
    authority: String,
    audience: String,
    minimal_update_interval: Option<Duration>,
    key_store: Arc<RwLock<KeyStore>>,
    client: reqwest::Client,
}

impl KeyManager {
    pub fn builder() -> KeyManagerBuilder {
        KeyManagerBuilder::default()
    }

    pub async fn update(&self) -> Result<(), JwksError> {
        let new_ks = KeyStore::new(&self.client, &self.authority, &self.audience).await?;
        let mut ks = self.key_store.write().await;
        *ks = new_ks;
        info!("Updated jwks from: {}", &self.authority);
        Ok(())
    }

    /// Validate the token, require claims in `T` to be present
    ///
    /// Verify correct `aud` and `exp`
    pub async fn validate_claims<T>(&self, token: &str) -> Result<TokenData<T>, TokenError>
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

        self.ensure_updated_key_store().await?;

        let ks = self.key_store.read().await;
        let key = ks.keys.get(kid).ok_or_else(|| {
            debug!(%kid, "Token refers to an unknown key.");
            TokenError::UnknownKeyId(kid.to_owned())
        })?;

        decode(token, &key.decoding, &key.validation).map_err(|error| {
            debug!(?error, "Token is malformed or does not pass validation.");
            TokenError::Invalid(error)
        })
    }

    async fn ensure_updated_key_store(&self) -> Result<(), TokenError> {
        let Some(minimal_interval) = self.minimal_update_interval else {
            return Ok(());
        };
        {
            let ks = self.key_store.read().await;
            let Some(last_updated) = ks.last_updated else {
                return Ok(());
            };
            if last_updated + minimal_interval > Instant::now() {
                return Ok(());
            }
        }
        self.update().await?;
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct KeyManagerBuilder {
    authority: String,
    audience: String,
    minimal_update_interval: Option<Duration>,
    key_store: Arc<RwLock<KeyStore>>,
    client: reqwest::Client,
}

impl KeyManagerBuilder {
    /// Create a new KeyManager that can fetch jwks from an authority
    /// `authority`: either url of an openid_configuration or a jwks_url
    /// `audience`: to be checked against the `aud` claim
    pub fn new(authority: String, audience: String) -> Self {
        Self {
            authority,
            audience,
            minimal_update_interval: None,
            key_store: Arc::new(RwLock::new(KeyStore::default())),
            client: reqwest::Client::default(),
        }
    }

    /// Do not update more often than `interval`
    pub fn minimal_update_interval(mut self, interval: u64) -> Self {
        self.minimal_update_interval = Some(Duration::from_secs(interval));
        self
    }

    /// Enables usage with externally provided `client`
    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    /// Fetch updated jwks now
    /// Required only if `with_periodical_update` or `with_minimal_update_interval` is not used
    pub async fn build(self) -> Result<KeyManager, JwksError> {
        let km = KeyManager {
            authority: self.authority,
            audience: self.audience,
            minimal_update_interval: self.minimal_update_interval,
            key_store: self.key_store,
            client: self.client,
        };
        km.update().await?;
        Ok(km)
    }
}
