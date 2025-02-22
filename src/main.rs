/*
 */

mod vault;

// use std::env;
// use kube::{Api, Client};
// use k8s_openapi::api::core::v1::Secret;
use tracing::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let vault = vault::client().await;
    info!("Vault settings: {:?}", vault.settings);
    vault::ensure().await;
    Ok(())
}
