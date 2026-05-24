use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultState {
    Uninitialized,
    Sealed,
    Ready,
    Unknown,
}

/// A validated Vault root token used for administrative operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootToken(String);

impl RootToken {
    pub fn new(token: String) -> Result<Self, VaulpnerError> {
        if token.is_empty() || token.len() < 10 {
            return Err(VaulpnerError::InvalidToken);
        }
        Ok(Self(token))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RootToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[redacted]")
    }
}

/// A single Vault unseal key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsealKey(String);

impl UnsealKey {
    pub fn new(key: String) -> Result<Self, VaulpnerError> {
        if key.is_empty() || key.len() < 10 {
            return Err(VaulpnerError::InvalidToken);
        }
        Ok(Self(key))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UnsealKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[redacted]")
    }
}

/// The full result of a Vault initialization: unseal keys and the root token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitResult {
    pub unseal_keys: Vec<UnsealKey>,
    pub root_token: RootToken,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VaulpnerError {
    #[error("Failed to build Vault client settings: {0}")]
    VaultClientBuild(String),

    #[error("Failed to create Vault client: {0}")]
    VaultClientCreate(String),

    #[error("Failed to infer Kubernetes config: {0}")]
    KubeConfigInfer(String),

    #[error("Failed to create Kubernetes client: {0}")]
    KubeClientCreate(String),

    #[error("Vault initialization failed: {0}")]
    VaultInit(String),

    #[error("Vault returned empty keys on initialization")]
    EmptyKeysResponse,

    #[error("Failed to create Kubernetes secret: {0}")]
    SecretCreate(String),

    #[error("Failed to get Kubernetes secret: {0}")]
    SecretGet(String),

    #[error("Failed to unseal Vault: {0}")]
    UnsealFailed(String),

    #[error("Root token is invalid: too short or empty")]
    InvalidToken,

    #[error("Failed to decode root token: {0}")]
    TokenDecode(String),

    #[error("Could not determine current namespace")]
    NamespaceNotFound,
}
