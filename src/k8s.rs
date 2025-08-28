/*
 */

use k8s_openapi::api::core::v1::Secret;
use base64::prelude::*;
use std::collections::BTreeMap;
use k8s_openapi::ByteString;

pub async fn client() -> Result<kube::Client, Box<dyn std::error::Error>> {
    let config = kube::Config::infer().await
        .map_err(|e| format!("Failed to infer Kubernetes config: {:?}", e))?;
    
    let client = kube::Client::try_from(config)
        .map_err(|e| format!("Failed to create Kubernetes client: {:?}", e))?;
    
    Ok(client)
}

pub async fn get_secret(client: &kube::Client, name: &str, namespace: &str) -> Result<Secret, kube::Error> {
    let secrets: kube::Api<Secret> = kube::Api::namespaced(client.clone(), namespace);
    secrets.get(name).await
}

pub async fn create_secret(client: &kube::Client, name: &str, namespace: &str, key: &str, value: &str) -> Result<Secret, kube::Error> {
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    let mut data = BTreeMap::new();
    data.insert(key.to_string(), ByteString(BASE64_STANDARD.encode(value).into_bytes()));

    let secret = Secret {
        metadata: ObjectMeta {
            name: Some(name.to_string()),
            namespace: Some(namespace.to_string()),
            ..ObjectMeta::default()
        },
        data: Some(data),
        ..Secret::default()
    };

    let secrets: kube::Api<Secret> = kube::Api::namespaced(client.clone(), namespace);
    secrets.create(&Default::default(), &secret).await
}

pub async fn namespace() -> Result<String, Box<dyn std::error::Error>> {
    // Method 1: Try to read from the service account token
    if let Ok(namespace) = std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/namespace") {
        return Ok(namespace.trim().to_string());
    }
    
    // Method 2: Check environment variable (set by downward API or manually)
    if let Ok(namespace) = std::env::var("POD_NAMESPACE") {
        return Ok(namespace);
    }
    
    // Method 3: Check KUBERNETES_NAMESPACE environment variable
    if let Ok(namespace) = std::env::var("KUBERNETES_NAMESPACE") {
        return Ok(namespace);
    }
    
    // Fallback to default namespace
    Ok("default".to_string())
}
