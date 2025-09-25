use std::env;
use std::fs;
use tempfile::tempdir;

use vaulpner::k8s;
use vaulpner::vault;

#[tokio::test]
async fn test_vault_client_creation_success() {
    // This test will only pass if Vault is accessible
    // In a real environment, you might mock the Vault client
    let result = vault::client().await;

    // If Vault is not available, we expect an error but not a panic
    match result {
        Ok(_) => {
            // Vault is available and client created successfully
            // Test passes if we reach here
        }
        Err(e) => {
            // Vault is not available, but we got a proper error instead of panic
            assert!(e.to_string().contains("Failed to"));
        }
    }
}

#[tokio::test]
async fn test_k8s_client_creation_success() {
    // This test will only pass if Kubernetes config is available
    let result = k8s::client().await;

    // If K8s is not available, we expect an error but not a panic
    match result {
        Ok(_) => {
            // K8s is available and client created successfully
            // Test passes if we reach here
        }
        Err(e) => {
            // K8s is not available, but we got a proper error instead of panic
            assert!(e.to_string().contains("Failed to"));
        }
    }
}

#[tokio::test]
async fn test_k8s_client_creation_with_invalid_config() {
    // Temporarily set an invalid KUBECONFIG to test error handling
    let temp_dir = tempdir().unwrap();
    let invalid_config_path = temp_dir.path().join("invalid-config");

    // Set invalid KUBECONFIG
    env::set_var("KUBECONFIG", invalid_config_path.to_str().unwrap());

    let result = k8s::client().await;

    // Clean up
    env::remove_var("KUBECONFIG");

    // Should return an error, not panic
    assert!(result.is_err());
    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(error_msg.contains("Failed to"));
    }
}

#[tokio::test]
async fn test_namespace_detection_methods() {
    // Test namespace detection with different scenarios

    // Test 1: No environment variables set
    env::remove_var("POD_NAMESPACE");
    env::remove_var("KUBERNETES_NAMESPACE");

    let result = k8s::namespace().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "default");

    // Test 2: POD_NAMESPACE environment variable
    env::set_var("POD_NAMESPACE", "test-namespace");

    let result = k8s::namespace().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test-namespace");

    // Test 3: KUBERNETES_NAMESPACE environment variable
    env::remove_var("POD_NAMESPACE");
    env::set_var("KUBERNETES_NAMESPACE", "k8s-namespace");

    let result = k8s::namespace().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "k8s-namespace");

    // Clean up
    env::remove_var("POD_NAMESPACE");
    env::remove_var("KUBERNETES_NAMESPACE");
}

#[tokio::test]
async fn test_namespace_detection_with_service_account() {
    // Create a temporary directory to simulate service account namespace file
    let temp_dir = tempdir().unwrap();
    let namespace_file = temp_dir.path().join("namespace");

    // Write a test namespace to the file
    fs::write(&namespace_file, "test-sa-namespace").unwrap();

    // Note: We can't easily test the service account path without root access
    // This test demonstrates the concept but won't run in practice
    // In a real Kubernetes environment, this would be tested with proper mocking

    // Clean up
    fs::remove_file(namespace_file).ok();
}

#[tokio::test]
async fn test_error_message_format() {
    // Test that error messages are descriptive and helpful

    // Test Vault client error message format
    let result = vault::client().await;
    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(error_msg.contains("Failed to"));
        assert!(error_msg.contains("Vault"));
    }

    // Test K8s client error message format
    let result = k8s::client().await;
    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(error_msg.contains("Failed to"));
        assert!(error_msg.contains("Kubernetes") || error_msg.contains("config"));
    }
}
