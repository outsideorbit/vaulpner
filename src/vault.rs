/*
 */

use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use vaultrs::api::sys::requests::StartInitializationRequest;
use tracing::*;
use vaultrs::{error::ClientError, api::sys::responses::StartInitializationResponse};

pub async fn client() -> VaultClient {
    let settings: vaultrs::client::VaultClientSettings = VaultClientSettingsBuilder::default()
        .build()
        .unwrap();
    VaultClient::new(settings).unwrap()
}


pub async fn initialize(vault: &VaultClient) -> Result<StartInitializationResponse, ClientError> {
    let mut opts = StartInitializationRequest::builder();
    let resp = vaultrs::sys::start_initialization(vault, 1, 1, Some(&mut opts)).await;
    resp
}

pub async fn ensure() {
    let vault: VaultClient = client().await;
    debug!("Vault settings: {:?}", vault.settings);
    let status = vault.status().await;
    match status {
        Ok(vaultrs::sys::ServerStatus::UNINITIALIZED) => {
            info!("Vault is uninitialized");
            let init_resp = initialize(&vault).await;
            debug!("Vault initialization response: {:?}", init_resp);
            if init_resp.is_err() {
                error!("Error initializing Vault: {:?}", init_resp.as_ref().err());
            }
        }
        Ok(vaultrs::sys::ServerStatus::SEALED) => {
            info!("Vault is sealed");
        }
        Ok(status) => {
            info!("Vault status: {:?}", status);
        }
        Err(ref e) => {
            error!("Error getting Vault status: {:?}", e);
        }
    }
}