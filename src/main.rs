use tracing::*;
use vaulpner::{
    adapters::{
        k8s::{self, K8sAdapter},
        vault::{self, VaultAdapter},
    },
    core::services::vault_lifecycle,
};

async fn run_with_retry(
    vault_client: &dyn vaulpner::core::ports::VaultRepository,
    k8s_client: &dyn vaulpner::core::ports::SecretStore,
    namespace: &str,
) {
    let max_attempts = 5;
    let mut attempt = 0;
    let mut delay_secs = 2u64;

    loop {
        match vault_lifecycle::ensure(vault_client, k8s_client, namespace).await {
            Ok(true) => {
                info!("Vault is ready");
                return;
            }
            Ok(false) => {
                info!("Vault is not ready yet");
            }
            Err(e) => {
                error!(error = %e, "Vault ensure failed");
            }
        }

        attempt += 1;
        if attempt >= max_attempts {
            error!(attempts = max_attempts, "Vault is not ready after max attempts, giving up");
            return;
        }

        info!(delay_secs = delay_secs, attempt = attempt, max_attempts = max_attempts, "Retrying");
        tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
        delay_secs = (delay_secs * 2).min(60);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let vault_client = VaultAdapter::new(vault::client().await?);
    info!("Vault client created");

    let k8s_client = K8sAdapter::new(k8s::client().await?);

    let namespace = k8s::namespace().await.unwrap_or_else(|e| {
        warn!(error = %e, "Could not determine namespace, falling back to 'default'");
        "default".to_string()
    });

    run_with_retry(&vault_client, &k8s_client, &namespace).await;

    Ok(())
}
