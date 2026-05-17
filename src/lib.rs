pub mod k8s;
pub mod vault;

use base64::Engine;
use tracing::{debug, info};
use vaultrs::client::Client;

const ROOT_TOKEN_SECRET_NAME: &str = "vault-root-token";
const ROOT_TOKEN_SECRET_KEY: &str = "root";
const MIN_TOKEN_LEN: usize = 10;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("kubernetes operation failed")]
    K8s(#[from] k8s::Error),

    #[error("vault operation failed")]
    Vault(#[from] vault::Error),

    #[error("vault status check failed")]
    Status(#[source] vaultrs::error::ClientError),

    #[error("root token secret '{name}' has no data")]
    SecretEmpty { name: &'static str },

    #[error("root token secret '{name}' is missing key '{key}'")]
    SecretMissingKey {
        name: &'static str,
        key: &'static str,
    },

    #[error("root token is not valid base64")]
    TokenNotBase64(#[source] base64::DecodeError),

    #[error("root token is not valid UTF-8")]
    TokenNotUtf8(#[source] std::string::FromUtf8Error),

    #[error("root token is empty or too short (< {MIN_TOKEN_LEN} bytes)")]
    TokenInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadyState {
    Ready,
    NotReady,
}

pub async fn ensure(
    vault_client: &vaultrs::client::VaultClient,
    k8s_client: &kube::Client,
) -> Result<ReadyState, Error> {
    let namespace = k8s::namespace();
    let status = vault_client.status().await.map_err(Error::Status)?;

    match status {
        vaultrs::sys::ServerStatus::UNINITIALIZED => {
            info!("Vault is uninitialized, initializing");
            let root_token = vault::initialize(vault_client).await?;
            k8s::create_secret(
                k8s_client,
                ROOT_TOKEN_SECRET_NAME,
                &namespace,
                ROOT_TOKEN_SECRET_KEY,
                &root_token,
            )
            .await?;
            info!(
                namespace = %namespace,
                secret = %ROOT_TOKEN_SECRET_NAME,
                "Root token stored in Kubernetes secret"
            );
            Ok(ReadyState::NotReady)
        }
        vaultrs::sys::ServerStatus::SEALED => {
            info!("Vault is sealed, retrieving root token to unseal");
            let secret =
                k8s::get_secret(k8s_client, ROOT_TOKEN_SECRET_NAME, &namespace).await?;
            debug!(name = %ROOT_TOKEN_SECRET_NAME, "Retrieved root token secret");

            let data = secret.data.ok_or(Error::SecretEmpty {
                name: ROOT_TOKEN_SECRET_NAME,
            })?;
            let bytes = data.get(ROOT_TOKEN_SECRET_KEY).ok_or(Error::SecretMissingKey {
                name: ROOT_TOKEN_SECRET_NAME,
                key: ROOT_TOKEN_SECRET_KEY,
            })?;
            let decoded = base64::prelude::BASE64_STANDARD
                .decode(&bytes.0)
                .map_err(Error::TokenNotBase64)?;
            let token = String::from_utf8(decoded).map_err(Error::TokenNotUtf8)?;
            if token.len() < MIN_TOKEN_LEN {
                return Err(Error::TokenInvalid);
            }

            vault::unseal(vault_client, &token).await?;
            info!("Vault unsealed");
            Ok(ReadyState::NotReady)
        }
        vaultrs::sys::ServerStatus::OK => {
            info!("Vault is unsealed");
            Ok(ReadyState::Ready)
        }
        other => {
            info!(status = ?other, "Vault unhandled status");
            Ok(ReadyState::NotReady)
        }
    }
}
