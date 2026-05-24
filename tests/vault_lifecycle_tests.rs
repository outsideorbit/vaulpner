use mockall::mock;
use vaulpner::core::{
    model::{InitResult, RootToken, UnsealKey, VaultState, VaulpnerError},
    ports::{BoxFuture, SecretStore, VaultRepository},
    services::vault_lifecycle,
};

mock! {
    pub VaultRepo {}
    impl VaultRepository for VaultRepo {
        fn status(&self) -> BoxFuture<'_, Result<VaultState, VaulpnerError>>;
        fn initialize(&self) -> BoxFuture<'_, Result<InitResult, VaulpnerError>>;
        fn unseal(&self, key: &UnsealKey) -> BoxFuture<'_, Result<(), VaulpnerError>>;
    }
}

mock! {
    pub SecretRepo {}
    impl SecretStore for SecretRepo {
        fn store_init_result(&self, name: &str, namespace: &str, result: &InitResult) -> BoxFuture<'_, Result<(), VaulpnerError>>;
        fn unseal_key(&self, name: &str, namespace: &str) -> BoxFuture<'_, Result<UnsealKey, VaulpnerError>>;
    }
}

fn init_result() -> InitResult {
    InitResult {
        unseal_keys: vec![UnsealKey::new("unsealkey1234567890".to_string()).unwrap()],
        root_token: RootToken::new("s.abcdefghij1234567890".to_string()).unwrap(),
    }
}

fn unseal_key() -> UnsealKey {
    UnsealKey::new("unsealkey1234567890".to_string()).unwrap()
}

#[tokio::test]
async fn ensure_returns_true_when_vault_ready() {
    let mut vault = MockVaultRepo::new();
    vault.expect_status().returning(|| Box::pin(async { Ok(VaultState::Ready) }));

    let secrets = MockSecretRepo::new();

    let result = vault_lifecycle::ensure(&vault, &secrets, "default").await;
    assert_eq!(result.unwrap(), true);
}

#[tokio::test]
async fn ensure_initializes_and_stores_token_when_uninitialized() {
    let mut vault = MockVaultRepo::new();
    vault.expect_status().returning(|| Box::pin(async { Ok(VaultState::Uninitialized) }));
    vault.expect_initialize().returning(|| Box::pin(async { Ok(init_result()) }));

    let mut secrets = MockSecretRepo::new();
    secrets.expect_store_init_result().returning(|_, _, _| Box::pin(async { Ok(()) }));

    let result = vault_lifecycle::ensure(&vault, &secrets, "default").await;
    assert_eq!(result.unwrap(), false);
}

#[tokio::test]
async fn ensure_retrieves_token_and_unseals_when_sealed() {
    let mut vault = MockVaultRepo::new();
    vault.expect_status().returning(|| Box::pin(async { Ok(VaultState::Sealed) }));
    vault.expect_unseal().returning(|_| Box::pin(async { Ok(()) }));

    let mut secrets = MockSecretRepo::new();
    secrets.expect_unseal_key().returning(|_, _| Box::pin(async { Ok(unseal_key()) }));

    let result = vault_lifecycle::ensure(&vault, &secrets, "default").await;
    assert_eq!(result.unwrap(), false);
}

#[tokio::test]
async fn ensure_returns_error_when_vault_status_fails() {
    let mut vault = MockVaultRepo::new();
    vault
        .expect_status()
        .returning(|| Box::pin(async { Err(VaulpnerError::VaultInit("connection refused".into())) }));

    let secrets = MockSecretRepo::new();

    let result = vault_lifecycle::ensure(&vault, &secrets, "default").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn ensure_returns_error_when_initialize_fails() {
    let mut vault = MockVaultRepo::new();
    vault.expect_status().returning(|| Box::pin(async { Ok(VaultState::Uninitialized) }));
    vault
        .expect_initialize()
        .returning(|| Box::pin(async { Err(VaulpnerError::VaultInit("init failed".into())) }));

    let secrets = MockSecretRepo::new();

    let result = vault_lifecycle::ensure(&vault, &secrets, "default").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn ensure_returns_error_when_get_token_fails() {
    let mut vault = MockVaultRepo::new();
    vault.expect_status().returning(|| Box::pin(async { Ok(VaultState::Sealed) }));

    let mut secrets = MockSecretRepo::new();
    secrets
        .expect_unseal_key()
        .returning(|_, _| Box::pin(async { Err(VaulpnerError::SecretGet("not found".into())) }));

    let result = vault_lifecycle::ensure(&vault, &secrets, "default").await;
    assert!(result.is_err());
}

