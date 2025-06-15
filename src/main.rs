/*
 */

mod k8s;
mod vault;

use tracing::*;
use vaultrs::client::Client;

pub async fn initialize_vault(vault_client: &vaultrs::client::VaultClient) -> Result<String, Box<dyn std::error::Error>> {
    match vault::initialize(vault_client).await {
        Ok(key) => {
            info!("Successfully initialized vault with key: {}", key);
            Ok(key)
        }
        Err(e) => {
            error!("Failed to initialize vault: {:?}", e);
            Err(Box::new(e))
        }
    }
}


pub async fn ensure(vault_client: &vaultrs::client::VaultClient, k8s_client: &kube::Client) -> bool {
    let mut result = false;
    let status = vault_client.status().await;
    match status {
        Ok(vaultrs::sys::ServerStatus::UNINITIALIZED) => {
            info!("Vault is uninitialized, initializing...");
        }
        Ok(status) => {
            info!("Vault status: {:?}", status);
            result = true;
        }
        Err(ref e) => {
            error!("Error getting Vault status: {:?}", e);
        }
    }
    result
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let vault = vault::client().await;
    info!("Vault settings: {:?}", vault.settings);
    let k8s_client = k8s::client().await;
    let max_count = 5; // Max wait incremented to 240 seconds (total time: 306 seconds)
    let mut count = 0;
    let mut count_increment = 2;
    while !ensure(&vault, &k8s_client).await {
        info!("Vault is not ready");
        if count >= max_count {
            error!("Vault is not ready after {} attempts", count);
            break;
        }
        count += 1;
        count_increment = count_increment * count;
        tokio::time::sleep(std::time::Duration::from_secs(count_increment)).await;
    }
    Ok(())
}
