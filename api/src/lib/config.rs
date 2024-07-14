// rivet-head-diary api
// src/lib/config.rs

// dependencies
use anyhow::Context;
use secrecy::Secret;
use shuttle_runtime::SecretStore;

// a struct to represent api configuration settings
pub struct ApiConfig {
    pub client_id: Secret<String>,
    pub client_secret: Secret<String>,
}

// implement the TryFrom trait for the ApiConfig struct
impl TryFrom<SecretStore> for ApiConfig {
    type Error = anyhow::Error;

    fn try_from(value: SecretStore) -> anyhow::Result<Self> {
        let client_id = value
            .get("CLIENT_ID")
            .context("CLIENT_ID secret should be set.")?
            .into();
        let client_secret = value
            .get("CLIENT_SECRET")
            .context("CLIENT_SECRET should be set.")?
            .into();

        Ok(Self {
            client_id,
            client_secret,
        })
    }
}
