/*
 */

use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

pub async fn client() -> VaultClient {
    let settings: vaultrs::client::VaultClientSettings = VaultClientSettingsBuilder::default()
        .build()
        .unwrap();
    VaultClient::new(settings).unwrap()
}

// pub async fn initialized(client: &VaultClient) -> bool {
//     false
// }