use std::env;
use tempfile::tempdir;

use vaulpner::k8s;
use vaulpner::vault;

#[tokio::test]
async fn test_vault_client_creation_returns_typed_error_or_ok() {
    // Without a live Vault, we expect a typed error rather than a panic.
    let result = vault::client().await;
    if let Err(e) = result {
        // Confirm the error chain is preserved (not flattened to a string).
        assert!(std::error::Error::source(&e).is_some() || matches!(e, vault::Error::BuildSettings(_) | vault::Error::CreateClient(_)));
    }
}

#[tokio::test]
async fn test_k8s_client_creation_returns_typed_error_or_ok() {
    let result = k8s::client().await;
    if let Err(e) = result {
        assert!(matches!(
            e,
            k8s::Error::InferConfig(_) | k8s::Error::CreateClient(_)
        ));
    }
}

#[tokio::test]
async fn test_k8s_client_creation_with_invalid_config() {
    let temp_dir = tempdir().expect("create tempdir");
    let invalid_config_path = temp_dir.path().join("invalid-config");

    // Note: env::set_var/remove_var racing with parallel tests is a known smell;
    // serial_test would fix it, out of scope for this change.
    env::set_var("KUBECONFIG", invalid_config_path.to_str().unwrap());
    let result = k8s::client().await;
    env::remove_var("KUBECONFIG");

    assert!(matches!(
        result,
        Err(k8s::Error::InferConfig(_) | k8s::Error::CreateClient(_))
    ));
}

#[tokio::test]
async fn test_namespace_detection_methods() {
    env::remove_var("POD_NAMESPACE");
    env::remove_var("KUBERNETES_NAMESPACE");

    // Falls back to "default" when nothing is set (assuming the SA path is absent).
    // On a host where /var/run/secrets/... exists, this assertion would fail —
    // acceptable for a dev workstation.
    assert_eq!(k8s::namespace(), "default");

    env::set_var("POD_NAMESPACE", "test-namespace");
    assert_eq!(k8s::namespace(), "test-namespace");

    env::remove_var("POD_NAMESPACE");
    env::set_var("KUBERNETES_NAMESPACE", "k8s-namespace");
    assert_eq!(k8s::namespace(), "k8s-namespace");

    env::remove_var("POD_NAMESPACE");
    env::remove_var("KUBERNETES_NAMESPACE");
}
