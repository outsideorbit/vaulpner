use std::process::ExitCode;
use tracing::{error, info};
use vaulpner::{ensure, k8s, vault, ReadyState};

const MAX_ATTEMPTS: u32 = 5;
const MAX_BACKOFF_SECS: u64 = 60;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    let vault_client = match vault::client().await {
        Ok(c) => c,
        Err(e) => {
            error!(error = ?e, "failed to create Vault client");
            return ExitCode::FAILURE;
        }
    };
    info!(address = %vault_client.settings.address, "Vault client created");

    let k8s_client = match k8s::client().await {
        Ok(c) => c,
        Err(e) => {
            error!(error = ?e, "failed to create Kubernetes client");
            return ExitCode::FAILURE;
        }
    };

    let mut attempt: u32 = 0;
    let mut delay_secs: u64 = 2;

    loop {
        match ensure(&vault_client, &k8s_client).await {
            Ok(ReadyState::Ready) => return ExitCode::SUCCESS,
            Ok(ReadyState::NotReady) => info!("Vault is not ready"),
            Err(e) => error!(error = ?e, "ensure failed"),
        }

        attempt += 1;
        if attempt >= MAX_ATTEMPTS {
            error!(attempts = attempt, "Vault did not become ready");
            return ExitCode::FAILURE;
        }

        delay_secs = delay_secs
            .saturating_mul(u64::from(attempt))
            .min(MAX_BACKOFF_SECS);
        tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
    }
}
