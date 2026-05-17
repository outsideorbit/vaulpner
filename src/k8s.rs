use base64::prelude::*;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::ByteString;
use std::collections::BTreeMap;

const SERVICE_ACCOUNT_NAMESPACE_PATH: &str =
    "/var/run/secrets/kubernetes.io/serviceaccount/namespace";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to infer Kubernetes config")]
    InferConfig(#[source] kube::config::InferConfigError),

    #[error("failed to create Kubernetes client")]
    CreateClient(#[source] kube::Error),

    #[error("Kubernetes API error")]
    Api(#[from] kube::Error),

    #[error("argument '{0}' must not be empty")]
    EmptyArg(&'static str),
}

pub async fn client() -> Result<kube::Client, Error> {
    let config = kube::Config::infer().await.map_err(Error::InferConfig)?;
    kube::Client::try_from(config).map_err(Error::CreateClient)
}

pub async fn get_secret(
    client: &kube::Client,
    name: &str,
    namespace: &str,
) -> Result<Secret, Error> {
    let secrets: kube::Api<Secret> = kube::Api::namespaced(client.clone(), namespace);
    Ok(secrets.get(name).await?)
}

fn validate_secret_args(
    name: &str,
    namespace: &str,
    key: &str,
    value: &str,
) -> Result<(), Error> {
    if name.is_empty() {
        return Err(Error::EmptyArg("name"));
    }
    if namespace.is_empty() {
        return Err(Error::EmptyArg("namespace"));
    }
    if key.is_empty() {
        return Err(Error::EmptyArg("key"));
    }
    if value.is_empty() {
        return Err(Error::EmptyArg("value"));
    }
    Ok(())
}

fn build_secret(name: &str, namespace: &str, key: &str, value: &str) -> Secret {
    let mut data = BTreeMap::new();
    data.insert(
        key.to_string(),
        ByteString(BASE64_STANDARD.encode(value).into_bytes()),
    );
    Secret {
        metadata: ObjectMeta {
            name: Some(name.to_string()),
            namespace: Some(namespace.to_string()),
            ..ObjectMeta::default()
        },
        data: Some(data),
        ..Secret::default()
    }
}

pub async fn create_secret(
    client: &kube::Client,
    name: &str,
    namespace: &str,
    key: &str,
    value: &str,
) -> Result<Secret, Error> {
    validate_secret_args(name, namespace, key, value)?;
    let secret = build_secret(name, namespace, key, value);
    let secrets: kube::Api<Secret> = kube::Api::namespaced(client.clone(), namespace);
    Ok(secrets.create(&Default::default(), &secret).await?)
}

pub fn namespace() -> String {
    if let Ok(ns) = std::fs::read_to_string(SERVICE_ACCOUNT_NAMESPACE_PATH) {
        return ns.trim().to_string();
    }
    if let Ok(ns) = std::env::var("POD_NAMESPACE") {
        return ns;
    }
    if let Ok(ns) = std::env::var("KUBERNETES_NAMESPACE") {
        return ns;
    }
    "default".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rejects_empty_name() {
        assert!(matches!(
            validate_secret_args("", "ns", "k", "v"),
            Err(Error::EmptyArg("name"))
        ));
    }

    #[test]
    fn validate_rejects_empty_namespace() {
        assert!(matches!(
            validate_secret_args("n", "", "k", "v"),
            Err(Error::EmptyArg("namespace"))
        ));
    }

    #[test]
    fn validate_rejects_empty_key() {
        assert!(matches!(
            validate_secret_args("n", "ns", "", "v"),
            Err(Error::EmptyArg("key"))
        ));
    }

    #[test]
    fn validate_rejects_empty_value() {
        assert!(matches!(
            validate_secret_args("n", "ns", "k", ""),
            Err(Error::EmptyArg("value"))
        ));
    }

    #[test]
    fn validate_accepts_all_present() {
        assert!(validate_secret_args("n", "ns", "k", "v").is_ok());
    }

    #[test]
    fn build_secret_base64_encodes_value() {
        let secret = build_secret("name", "ns", "root", "hvs.example");
        let data = secret.data.expect("data populated");
        let stored = data.get("root").expect("key populated");
        let decoded = BASE64_STANDARD
            .decode(&stored.0)
            .expect("stored bytes are base64");
        assert_eq!(decoded, b"hvs.example");
        assert_eq!(secret.metadata.name.as_deref(), Some("name"));
        assert_eq!(secret.metadata.namespace.as_deref(), Some("ns"));
    }
}
