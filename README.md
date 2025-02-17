# vaulpner
Small utility to run as a sidecar in a Kubernetes deployment for Vault in development type environments.  
It will ensure that the Vault is initialized and unsealed when deployed to simplify the process of setting up 
the Vault for development purposes.

The utility will check if Vault is unsealed and initialized, and if not, it will initialize and unseal the Vault.
Once the Vault is initialized and unsealed, it will write the root token to a `Secret` in the same namespace as the 
deployment. The `Secret` will be named `vault-root-token` and the key will be `root-token`.

## Dependencies

```
pkg-config
libssl-dev
```