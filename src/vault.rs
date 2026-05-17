use tracing::debug;
use vaultrs::api::sys::requests::StartInitializationRequest;
use vaultrs::api::sys::responses::StartInitializationResponse;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::error::ClientError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to build Vault client settings")]
    BuildSettings(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to create Vault client")]
    CreateClient(#[source] ClientError),

    #[error("Vault returned no unseal keys")]
    NoUnsealKeys,

    #[error(transparent)]
    Vault(#[from] ClientError),
}

pub async fn client() -> Result<VaultClient, Error> {
    let settings = VaultClientSettingsBuilder::default()
        .build()
        .map_err(|e| Error::BuildSettings(Box::new(e)))?;

    VaultClient::new(settings).map_err(Error::CreateClient)
}

pub async fn start_initialization(
    vault: &VaultClient,
) -> Result<StartInitializationResponse, ClientError> {
    let mut opts = StartInitializationRequest::builder();
    vaultrs::sys::start_initialization(vault, 1, 1, Some(&mut opts)).await
}

pub async fn initialize(vault: &VaultClient) -> Result<String, Error> {
    let response = start_initialization(vault).await?;
    debug!("Vault initialization complete");
    response.keys.first().cloned().ok_or(Error::NoUnsealKeys)
}

pub async fn unseal(vault: &VaultClient, key: &str) -> Result<(), Error> {
    vaultrs::sys::unseal(vault, Some(key.to_string()), None, None).await?;
    debug!("Vault unseal request accepted");
    Ok(())
}
