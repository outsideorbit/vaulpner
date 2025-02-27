/*
 */

mod vault;

use tracing::*;
use vaultrs::client::Client;

pub async fn ensure(vault: &vaultrs::client::VaultClient) -> bool {
    let mut result = false;
    let status = vault.status().await;
    match status {
        Ok(vaultrs::sys::ServerStatus::UNINITIALIZED) => {
            info!("Vault is uninitialized");
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
    let max_count = 10;
    let mut count = 0;
    let mut count_increment = 2;
    while !ensure(&vault).await {
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
