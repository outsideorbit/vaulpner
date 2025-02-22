/*
 */

use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use tracing::*;

pub async fn client() -> VaultClient {
    let settings: vaultrs::client::VaultClientSettings = VaultClientSettingsBuilder::default()
        .build()
        .unwrap();
    VaultClient::new(settings).unwrap()
}



pub async fn ensure() {
    let vault: VaultClient = client().await;
    info!("Vault settings: {:?}", vault.settings);
    let status = vault.status().await;
    match status {
        Ok(vaultrs::sys::ServerStatus::UNINITIALIZED) => {
            info!("Vault is uninitialized");
            let secret_shares = 1; // Define secret_shares
            let secret_threshold = 1; // Define secret_threshold
            let opts = vaultrs::
            let init_resp = vaultrs::sys::start_initialization(
                &vault,
                secret_shares,
                secret_threshold, _).await;
            info!("Vault initialization response: {:?}", init_resp);
        }
        Ok(vaultrs::sys::ServerStatus::SEALED) => {
            info!("Vault is sealed");
        }
        Ok(status) => {
            info!("Vault status: {:?}", status);
        }
        Err(e) => {
            error!("Error getting Vault status: {:?}", e);
        }
    }
}