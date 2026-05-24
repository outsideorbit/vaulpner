use tracing::*;
use vaultrs::api::sys::requests::StartInitializationRequest;
use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};

use crate::core::{
    model::{InitResult, RootToken, UnsealKey, VaultState, VaulpnerError},
    ports::{BoxFuture, VaultRepository},
};

/// Builds a Vault client from environment / default settings.
pub async fn client() -> Result<VaultClient, VaulpnerError> {
    let settings = VaultClientSettingsBuilder::default()
        .build()
        .map_err(|e| VaulpnerError::VaultClientBuild(e.to_string()))?;

    VaultClient::new(settings).map_err(|e| VaulpnerError::VaultClientCreate(e.to_string()))
}

/// Adapter wrapping `VaultClient` to implement the `VaultRepository` port.
pub struct VaultAdapter(VaultClient);

impl VaultAdapter {
    pub fn new(client: VaultClient) -> Self {
        Self(client)
    }
}

impl std::fmt::Debug for VaultAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VaultAdapter").finish_non_exhaustive()
    }
}

impl VaultRepository for VaultAdapter {
    fn status(&self) -> BoxFuture<'_, Result<VaultState, VaulpnerError>> {
        Box::pin(async move {
            match Client::status(&self.0).await {
                Ok(vaultrs::sys::ServerStatus::UNINITIALIZED) => Ok(VaultState::Uninitialized),
                Ok(vaultrs::sys::ServerStatus::SEALED) => Ok(VaultState::Sealed),
                Ok(vaultrs::sys::ServerStatus::OK) => Ok(VaultState::Ready),
                Ok(s) => {
                    debug!(status = ?s, "Unhandled Vault status");
                    Ok(VaultState::Unknown)
                }
                Err(e) => Err(VaulpnerError::VaultInit(e.to_string())),
            }
        })
    }

    fn initialize(&self) -> BoxFuture<'_, Result<InitResult, VaulpnerError>> {
        Box::pin(async move {
            let mut opts = StartInitializationRequest::builder();
            let resp = vaultrs::sys::start_initialization(&self.0, 1, 1, Some(&mut opts))
                .await
                .map_err(|e| VaulpnerError::VaultInit(e.to_string()))?;

            debug!(response = ?resp, "Vault initialized successfully");

            if resp.keys.is_empty() {
                return Err(VaulpnerError::EmptyKeysResponse);
            }

            let unseal_keys = resp
                .keys
                .into_iter()
                .map(UnsealKey::new)
                .collect::<Result<Vec<_>, _>>()?;

            let root_token = RootToken::new(resp.root_token)?;

            Ok(InitResult { unseal_keys, root_token })
        })
    }

    fn unseal(&self, key: &UnsealKey) -> BoxFuture<'_, Result<(), VaulpnerError>> {
        let key_str = key.as_str().to_string();
        Box::pin(async move {
            let resp = vaultrs::sys::unseal(&self.0, Some(key_str), None, None)
                .await
                .map_err(|e| VaulpnerError::UnsealFailed(e.to_string()))?;
            debug!(response = ?resp, "Vault unseal response");
            Ok(())
        })
    }
}

