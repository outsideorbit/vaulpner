use crate::core::{
    model::{VaultState, VaulpnerError},
    ports::{SecretStore, VaultRepository},
};

const SECRET_NAME: &str = "vault-init";

/// Core business logic for ensuring Vault is initialized and unsealed.
/// Takes port trait objects — has no knowledge of vaultrs or kube.
pub async fn ensure(
    vault: &dyn VaultRepository,
    secrets: &dyn SecretStore,
    namespace: &str,
) -> Result<bool, VaulpnerError> {
    match vault.status().await? {
        VaultState::Uninitialized => {
            let result = vault.initialize().await?;
            secrets.store_init_result(SECRET_NAME, namespace, &result).await?;
            Ok(false)
        }
        VaultState::Sealed => {
            let key = secrets.unseal_key(SECRET_NAME, namespace).await?;
            vault.unseal(&key).await?;
            Ok(false)
        }
        VaultState::Ready => Ok(true),
        VaultState::Unknown => Ok(false),
    }
}
