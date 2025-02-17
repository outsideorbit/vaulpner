/*
 */

mod vault;

// use std::env;
// use kube::{Api, Client};
// use k8s_openapi::api::core::v1::Secret;
use tracing::*;
use vaultrs::client::{Client, VaultClient};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let vault: VaultClient = vault::client().await;
    info!(("Vault client initialized: {:?}", vault));
    Ok(())
}
