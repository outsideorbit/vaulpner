# vaulpner

A Kubernetes sidecar utility that automatically initializes and unseals HashiCorp Vault in development environments. This tool simplifies Vault setup by handling the initialization process with a single unseal key and securely storing the root token in Kubernetes secrets.

## 🚀 Features

- **Automatic Vault Initialization**: Detects uninitialized Vault instances and initializes them with a single unseal key
- **Automatic Vault Unsealing**: Retrieves and uses stored root tokens to unseal Vault
- **Secure Token Storage**: Stores root tokens in Kubernetes secrets for persistence
- **Kubernetes Native**: Designed to run as a sidecar container in Kubernetes deployments
- **Development Focused**: Optimized for development and testing environments
- **Retry Logic**: Implements exponential backoff for reliable operation

## 📋 Prerequisites

### System Dependencies
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install pkg-config libssl-dev

# CentOS/RHEL
sudo yum install pkgconfig openssl-devel

# macOS
brew install pkg-config openssl
```

### Kubernetes Requirements
- Kubernetes cluster with Vault deployed
- Service account with permissions to create/read secrets
- Network connectivity between vaulpner and Vault

## 🛠️ Installation

### Using Docker
```bash
docker pull ghcr.io/outsideorbit/vaulpner:latest
```

### From Source
```bash
git clone https://github.com/outsideorbit/vaulpner.git
cd vaulpner
cargo build --release
```

## 🚀 Quick Start

### 1. Deploy Vault
First, deploy Vault to your Kubernetes cluster:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: vault
  namespace: vault
spec:
  replicas: 1
  selector:
    matchLabels:
      app: vault
  template:
    metadata:
      labels:
        app: vault
    spec:
      containers:
      - name: vault
        image: hashicorp/vault:latest
        ports:
        - containerPort: 8200
        env:
        - name: VAULT_DEV_ROOT_TOKEN_ID
          value: "dev-token"
        - name: VAULT_DEV_LISTEN_ADDRESS
          value: "0.0.0.0:8200"
        command: ["vault", "server", "-dev"]
```

### 2. Deploy vaulpner as Sidecar
Add vaulpner as a sidecar container to your Vault deployment:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: vault-with-vaulpner
  namespace: vault
spec:
  replicas: 1
  selector:
    matchLabels:
      app: vault
  template:
    metadata:
      labels:
        app: vault
    spec:
      serviceAccountName: vault-service-account
      containers:
      - name: vault
        image: hashicorp/vault:latest
        ports:
        - containerPort: 8200
        env:
        - name: VAULT_DEV_LISTEN_ADDRESS
          value: "0.0.0.0:8200"
        command: ["vault", "server", "-dev"]
      - name: vaulpner
        image: ghcr.io/outsideorbit/vaulpner:latest
        env:
        - name: VAULT_ADDR
          value: "http://localhost:8200"
        - name: POD_NAMESPACE
          valueFrom:
            fieldRef:
              fieldPath: metadata.namespace
```

### 3. Create Service Account
Create a service account with the necessary permissions:

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: vault-service-account
  namespace: vault
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: vault-secret-manager
  namespace: vault
rules:
- apiGroups: [""]
  resources: ["secrets"]
  verbs: ["get", "create", "update", "patch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: vault-secret-manager
  namespace: vault
subjects:
- kind: ServiceAccount
  name: vault-service-account
  namespace: vault
roleRef:
  kind: Role
  name: vault-secret-manager
  apiGroup: rbac.authorization.k8s.io
```

## ⚙️ Configuration

### Important: Single Unseal Key Limitation

**vaulpner only supports Vault configurations with a single unseal key.** This is intentional for development environments where simplicity is prioritized over high availability.

### Vault Configuration Options

#### Option 1: Single Key (Recommended for Development)
Configure Vault with a single unseal key for development environments:

```bash
# Initialize Vault with 1 key threshold and 1 key share
vault operator init -key-shares=1 -key-threshold=1
```

#### Option 2: Dev Mode (No Unsealing Required)
For pure development/testing, use Vault's dev mode which doesn't require unsealing:

```bash
# Start Vault in dev mode (no unsealing needed)
vault server -dev -dev-root-token-id="root"
```

**Note:** vaulpner is not needed when using dev mode as Vault starts unsealed.

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `VAULT_ADDR` | Vault server address | `http://localhost:8200` | Yes |
| `POD_NAMESPACE` | Kubernetes namespace | Auto-detected | No |
| `KUBERNETES_NAMESPACE` | Alternative namespace detection | Auto-detected | No |
| `RUST_LOG` | Log level | `info` | No |

### Kubernetes Secret

vaulpner stores the Vault root token in a Kubernetes secret with the following structure:

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: vault-root-token
  namespace: vault
type: Opaque
data:
  root: <base64-encoded-root-token>
```

## 🔧 Usage

### Basic Usage
```bash
# Run with default settings
vaulpner

# Run with custom Vault address
VAULT_ADDR=http://vault.example.com:8200 vaulpner

# Run with debug logging
RUST_LOG=debug vaulpner
```

### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-app
spec:
  template:
    spec:
      containers:
      - name: my-app
        image: my-app:latest
        # ... your app configuration
      - name: vaulpner
        image: ghcr.io/outsideorbit/vaulpner:latest
        env:
        - name: VAULT_ADDR
          value: "http://vault:8200"
```

## 🔍 How It Works

1. **Status Check**: vaulpner checks if Vault is initialized and unsealed
2. **Initialization**: If uninitialized, it initializes Vault with a single unseal key (key-shares=1, key-threshold=1)
3. **Token Storage**: Stores the root token in a Kubernetes secret named `vault-root-token`
4. **Unsealing**: If sealed, retrieves the root token from the secret and unseals Vault using the single key
5. **Retry Logic**: Implements exponential backoff with a maximum of 5 attempts

**Important:** This approach uses a single unseal key for simplicity in development environments. For production use, consider using Vault's auto-unseal features or multiple unseal keys with proper key management.

## 🐛 Troubleshooting

### Common Issues

#### Multiple Unseal Keys Error
If you see errors about multiple unseal keys, ensure Vault is configured with a single key:

```bash
# Check current Vault configuration
vault status

# Re-initialize with single key (WARNING: This will reset Vault)
vault operator init -key-shares=1 -key-threshold=1
```

#### Vault Not Accessible
```bash
# Check if Vault is running
kubectl get pods -n vault

# Check Vault logs
kubectl logs -n vault deployment/vault

# Verify network connectivity
kubectl exec -it <vaulpner-pod> -- curl http://vault:8200/v1/sys/health
```

#### Permission Denied
```bash
# Check service account permissions
kubectl auth can-i create secrets --as=system:serviceaccount:vault:vault-service-account

# Verify RBAC configuration
kubectl get rolebinding vault-secret-manager -n vault -o yaml
```

#### Secret Not Found
```bash
# Check if secret exists
kubectl get secret vault-root-token -n vault

# Check secret contents
kubectl get secret vault-root-token -n vault -o yaml
```

### Debug Mode
Enable debug logging to see detailed information:

```yaml
env:
- name: RUST_LOG
  value: "debug"
```

### Logs
```bash
# View vaulpner logs
kubectl logs -n vault deployment/vault-with-vaulpner -c vaulpner

# Follow logs in real-time
kubectl logs -n vault deployment/vault-with-vaulpner -c vaulpner -f
```

## 🔒 Security Considerations

- **Development Only**: This tool is designed for development environments
- **Root Token Storage**: Root tokens are stored in Kubernetes secrets (base64 encoded)
- **Network Security**: Ensure Vault is not exposed to untrusted networks
- **RBAC**: Use least-privilege principles for service account permissions
- **Token Rotation**: Consider implementing token rotation for production use

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/outsideorbit/vaulpner/issues)
- **Discussions**: [GitHub Discussions](https://github.com/outsideorbit/vaulpner/discussions)
- **Documentation**: [Wiki](https://github.com/outsideorbit/vaulpner/wiki)

## 📚 Additional Resources

- [HashiCorp Vault Documentation](https://www.vaultproject.io/docs)
- [Kubernetes Secrets Documentation](https://kubernetes.io/docs/concepts/configuration/secret/)
- [Rust Documentation](https://doc.rust-lang.org/)