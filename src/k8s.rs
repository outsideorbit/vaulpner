use kube::Api;
use k8s_openapi::api::core::v1::Secret;
use tracing::*;

pub async fn client() -> kube::Client {
    let config = kube::Config::infer().await.unwrap();
    let client = kube::Client::try_from(config).unwrap();
    client
}

pub async fn get_secret(client: &kube::Client, name: &str, namespace: &str) -> Result<Option<Secret>, kube::Error> {
    let api: Api<Secret> = Api::namespaced(client.clone(), namespace);
    // Try to get the secret, if it doesn't exist, return None
    match api.get_opt(name).await {
        Ok(secret) => Ok(secret),
        Err(e) => {
            error!("Error getting secret {}/{}: {:?}", namespace, name, e);
            Err(e)
        }
    }
}

// pub async fn update_secret(client: &kube::Client, name: &str, namespace: &str) -> Result<Option<Secret>, kube::Error> {
//     Err(())
// }
