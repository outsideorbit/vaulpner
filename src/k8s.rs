/*
 */

 use k8s_openapi::api::core::v1::Secret;

pub async fn client() -> kube::Client {
    let config = kube::Config::infer().await.unwrap();
    let client = kube::Client::try_from(config).unwrap();
    client
}

pub async fn get_secret(client: &kube::Client, name: &str, namespace: &str) -> Result<Secret, kube::Error> {
    let secrets: kube::Api<Secret> = kube::Api::namespaced(client.clone(), namespace);
    secrets.get(name).await
}

pub async fn create_secret(client: &kube::Client, name: &str, namespace: &str, key: &str, value: &str) -> Result<Secret, kube::Error> {
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    use std::collections::BTreeMap;

    let mut data = BTreeMap::new();
    data.insert(key.to_string(), base64::encode(value));

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
