/*
 */

use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use vaultrs::api::sys::requests::StartInitializationRequest;
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

// pub async fn unseal(vault: &VaultClient) -> Result<(), ClientError>  {
//     let resp = vaultrs::sys::unseal(vault, "1234567890").await;
//     resp
// }
