use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::ByteString;
use std::collections::BTreeMap;
use tracing::*;

use crate::core::{
    model::{InitResult, UnsealKey, VaulpnerError},
    ports::{BoxFuture, SecretStore},
};

const KEY_ROOT_TOKEN: &str = "root-token";
const KEY_UNSEAL_PREFIX: &str = "unseal-key-";

/// Builds a Kubernetes client from the in-cluster or kubeconfig environment.
pub async fn client() -> Result<kube::Client, VaulpnerError> {
    let config = kube::Config::infer()
        .await
        .map_err(|e| VaulpnerError::KubeConfigInfer(e.to_string()))?;

    kube::Client::try_from(config).map_err(|e| VaulpnerError::KubeClientCreate(e.to_string()))
}

/// Resolves the current Kubernetes namespace via service account file or environment variables.
pub async fn namespace() -> Result<String, VaulpnerError> {
    if let Ok(ns) = std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/namespace") {
        return Ok(ns.trim().to_string());
    }
    if let Ok(ns) = std::env::var("POD_NAMESPACE") {
        return Ok(ns);
    }
    if let Ok(ns) = std::env::var("KUBERNETES_NAMESPACE") {
        return Ok(ns);
    }
    Err(VaulpnerError::NamespaceNotFound)
}

/// Adapter wrapping `kube::Client` to implement the `SecretStore` port.
pub struct K8sAdapter(kube::Client);

impl K8sAdapter {
    pub fn new(client: kube::Client) -> Self {
        Self(client)
    }
}

impl std::fmt::Debug for K8sAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("K8sAdapter").finish_non_exhaustive()
    }
}

impl SecretStore for K8sAdapter {
    fn store_init_result(&self, name: &str, namespace: &str, result: &InitResult) -> BoxFuture<'_, Result<(), VaulpnerError>> {
        let name = name.to_string();
        let namespace = namespace.to_string();
        let mut data: BTreeMap<String, ByteString> = BTreeMap::new();

        for (i, key) in result.unseal_keys.iter().enumerate() {
            data.insert(
                format!("{}{}", KEY_UNSEAL_PREFIX, i),
                ByteString(key.as_str().as_bytes().to_vec()),
            );
        }
        data.insert(
            KEY_ROOT_TOKEN.to_string(),
            ByteString(result.root_token.as_str().as_bytes().to_vec()),
        );

        Box::pin(async move {
            if name.is_empty() || namespace.is_empty() {
                warn!(secret_name = %name, namespace = %namespace, "store_init_result called with empty name or namespace");
            }

            let secret = Secret {
                metadata: ObjectMeta {
                    name: Some(name),
                    namespace: Some(namespace.clone()),
                    ..ObjectMeta::default()
                },
                data: Some(data),
                ..Secret::default()
            };

            let api: kube::Api<Secret> = kube::Api::namespaced(self.0.clone(), &namespace);
            api.create(&Default::default(), &secret)
                .await
                .map_err(|e| VaulpnerError::SecretCreate(e.to_string()))?;

            Ok(())
        })
    }

    fn unseal_key(&self, name: &str, namespace: &str) -> BoxFuture<'_, Result<UnsealKey, VaulpnerError>> {
        let name = name.to_string();
        let namespace = namespace.to_string();
        Box::pin(async move {
            let api: kube::Api<Secret> = kube::Api::namespaced(self.0.clone(), &namespace);
            let secret = api
                .get(&name)
                .await
                .map_err(|e| VaulpnerError::SecretGet(e.to_string()))?;

            let data = secret.data.ok_or_else(|| VaulpnerError::SecretGet("secret has no data".into()))?;

            let key = format!("{}0", KEY_UNSEAL_PREFIX);
            let bytes = data
                .get(&key)
                .ok_or_else(|| VaulpnerError::SecretGet(format!("key '{}' not found in secret", key)))?;

            let key_str = String::from_utf8(bytes.0.clone())
                .map_err(|e| VaulpnerError::TokenDecode(e.to_string()))?;

            UnsealKey::new(key_str)
        })
    }
}


