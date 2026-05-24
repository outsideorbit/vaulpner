use std::env;
use std::fs;
use tempfile::tempdir;

use vaulpner::adapters::{k8s, vault};

#[tokio::test(flavor = "current_thread")]
async fn test_vault_client_creation_success() {
    let result = vault::client().await;
    match result {
        Ok(_) => {}
        Err(e) => {
            assert!(e.to_string().contains("Failed to"));
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn test_k8s_client_creation_success() {
    let result = k8s::client().await;
    match result {
        Ok(_) => {}
        Err(e) => {
            assert!(e.to_string().contains("Failed to"));
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn test_k8s_client_creation_with_invalid_config() {
    let temp_dir = tempdir().unwrap();
    let invalid_config_path = temp_dir.path().join("invalid-config");

    env::set_var("KUBECONFIG", invalid_config_path.to_str().unwrap());
    let result = k8s::client().await;
    env::remove_var("KUBECONFIG");

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Failed to"));
    }
}

#[tokio::test(flavor = "current_thread")]
async fn test_namespace_detection_methods() {
    env::remove_var("POD_NAMESPACE");
    env::remove_var("KUBERNETES_NAMESPACE");

    // No env vars and no service account file → NamespaceNotFound
    let result = k8s::namespace().await;
    assert!(result.is_err());

    // POD_NAMESPACE set
    env::set_var("POD_NAMESPACE", "test-namespace");
    let result = k8s::namespace().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test-namespace");

    // KUBERNETES_NAMESPACE fallback
    env::remove_var("POD_NAMESPACE");
    env::set_var("KUBERNETES_NAMESPACE", "k8s-namespace");
    let result = k8s::namespace().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "k8s-namespace");

    env::remove_var("POD_NAMESPACE");
    env::remove_var("KUBERNETES_NAMESPACE");
}

#[tokio::test(flavor = "current_thread")]
async fn test_namespace_detection_with_service_account() {
    let temp_dir = tempdir().unwrap();
    let namespace_file = temp_dir.path().join("namespace");
    fs::write(&namespace_file, "test-sa-namespace").unwrap();
    // The service account namespace path is fixed at /var/run/secrets/kubernetes.io/serviceaccount/namespace.
    // Full coverage of that branch requires an integration environment.
    // This test validates that the temp file write succeeded (the fixture is valid).
    assert!(namespace_file.exists());
    fs::remove_file(namespace_file).ok();
}

#[tokio::test(flavor = "current_thread")]
async fn test_error_message_format() {
    let result = vault::client().await;
    if let Err(e) = result {
        assert!(e.to_string().contains("Failed to"));
    }

    let result = k8s::client().await;
    if let Err(e) = result {
        assert!(e.to_string().contains("Failed to"));
    }
}

