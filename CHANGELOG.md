# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation with deployment examples
- Troubleshooting guide for common issues
- Contributing guidelines
- Configuration documentation

### Changed
- Updated README with detailed usage instructions
- Fixed documentation inconsistency with secret key name
- Enhanced Cargo.toml metadata

## [0.1.0] - 2024-01-XX

### Added
- Initial release of vaulpner
- Automatic Vault initialization and unsealing
- Kubernetes secret integration for root token storage
- Exponential backoff retry logic
- Comprehensive error handling
- Multi-platform container support (linux/amd64, linux/arm64)
- CI/CD pipeline with semantic versioning
- Security scanning and SBOM generation
- Comprehensive test suite

### Features
- **Vault Initialization**: Automatically initializes uninitialized Vault instances
- **Vault Unsealing**: Retrieves stored root tokens and unseals Vault
- **Secret Management**: Stores root tokens in Kubernetes secrets
- **Kubernetes Integration**: Designed as a sidecar container
- **Development Focused**: Optimized for development environments
- **Retry Logic**: Implements exponential backoff for reliability

### Dependencies
- `vaultrs` 0.7.3 - Vault API client
- `kube` 0.98.0 - Kubernetes client
- `k8s-openapi` 0.24.0 - Kubernetes API types
- `tokio` 1.43.1 - Async runtime
- `tracing` 0.1.41 - Structured logging
- `base64` 0.22.1 - Base64 encoding/decoding

### Security
- No hardcoded secrets or credentials
- Secure token storage in Kubernetes secrets
- Input validation and sanitization
- Base64 encoding for secret data
- RBAC-compliant service account permissions

### Testing
- Unit tests for all public functions
- Integration tests for complete workflows
- Error handling and edge case testing
- Mock-based testing for external dependencies

### CI/CD
- GitHub Actions workflow for build and push
- Semantic versioning with conventional commits
- Multi-platform container builds
- Security scanning with Trivy
- SBOM and provenance generation
- Automated testing and linting

## [0.0.1] - 2024-01-XX

### Added
- Initial project setup
- Basic Vault client implementation
- Kubernetes client implementation
- Core initialization and unsealing logic
- Basic error handling
- Initial test framework
