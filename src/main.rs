/*
 */

mod k8s;
mod vault;

use tracing::*;
use vaultrs::client::Client;
use base64::Engine;

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

pub async fn get_current_namespace() -> String {
    match k8s::namespace().await {
        Ok(ns) => ns,
        Err(e) => {
            error!("Failed to get namespace: {:?}", e);
            "default".to_string()
        }
    }
}

pub async fn ensure(vault_client: &vaultrs::client::VaultClient, k8s_client: &kube::Client) -> bool {
    let mut result = false;
    let namespace = get_current_namespace().await;
    let status = vault_client.status().await;
    match status {
        Ok(vaultrs::sys::ServerStatus::UNINITIALIZED) => {
            info!("Vault is uninitialized, initializing...");
            // Initialize Vault
            match vault::initialize(vault_client).await {
                Ok(root_token) => {
                    info!("Vault initialized, storing root token in Kubernetes secret");
                    match k8s::create_secret(k8s_client, "vault-root-token", &namespace, "root", &root_token).await {
                        Ok(_) => info!("Root token stored in secret 'vault-root-token' (key: 'root') in namespace {}", namespace),
                        Err(e) => error!("Failed to create secret: {:?}", e),
                    }
                }
                Err(e) => {
                    error!("Failed to initialize Vault: {:?}", e);
                }
            }
        }
        Ok(vaultrs::sys::ServerStatus::SEALED) => {
            info!("Vault is sealed");
            // Pull the secret
            match k8s::get_secret(k8s_client, "vault-root-token", &namespace).await {
                Ok(secret) => {
                    info!("Retrieved root token secret: {:?}", secret.data);
                    if let Some(data) = secret.data {
                        if let Some(root_token_bytes) = data.get("root") {
                            // Decode base64 bytes to get the actual root token
                            let root_token = match base64::prelude::BASE64_STANDARD.decode(&root_token_bytes.0) {
                                Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                                Err(e) => {
                                    error!("Failed to decode root token from base64: {:?}", e);
                                    return result;
                                }
                            };
                            debug!("Decoded root token: {}", root_token);                            
                            // Unseal Vault
                            match vault::unseal(vault_client, &root_token).await {
                                Ok(_) => info!("Vault unsealed successfully"),
                                Err(e) => error!("Failed to unseal Vault: {:?}", e),
                            }
                        } else {
                            error!("Root token not found in secret data");
                        }
                    } else {
                        error!("Secret has no data");
                    }
                }
                Err(e) => error!("Failed to get root token secret: {:?}", e),
            };
        }
        Ok(vaultrs::sys::ServerStatus::OK) => {
            info!("Vault is unsealed");
            // Vault is unsealed and ready to go
            result = true;
        }
        Ok(status) => {
            info!("Vault unhandled status: {:?}", status);
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
    
    // Create clients with proper error handling
    let vault = vault::client().await?;
    info!("Vault settings: {:?}", vault.settings);
    
    let k8s_client = k8s::client().await?;
    
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
