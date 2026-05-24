use std::future::Future;
use std::pin::Pin;

use crate::core::model::{InitResult, UnsealKey, VaultState, VaulpnerError};

/// Shared return type for all async port methods.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Port for interacting with Vault. Implemented by adapters in `crate::adapters::vault`.
pub trait VaultRepository: Send + Sync {
    fn status(&self) -> BoxFuture<'_, Result<VaultState, VaulpnerError>>;
    fn initialize(&self) -> BoxFuture<'_, Result<InitResult, VaulpnerError>>;
    fn unseal(&self, key: &UnsealKey) -> BoxFuture<'_, Result<(), VaulpnerError>>;
}

/// Port for interacting with a secret store. Implemented by adapters in `crate::adapters::k8s`.
pub trait SecretStore: Send + Sync {
    /// Persists the full initialization result (all unseal keys + root token) as a K8s secret.
    fn store_init_result(&self, name: &str, namespace: &str, result: &InitResult) -> BoxFuture<'_, Result<(), VaulpnerError>>;
    /// Retrieves the first unseal key from a previously stored initialization secret.
    fn unseal_key(&self, name: &str, namespace: &str) -> BoxFuture<'_, Result<UnsealKey, VaulpnerError>>;
}

