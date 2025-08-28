# GitHub Workflows

This directory contains GitHub workflows for building, pushing, and securing container images.

## Workflows Overview

### 1. Build and Push Container Image (`build-and-push.yml`)
Builds the vaulpner container image and pushes it to GitHub Container Registry (GHCR).

### 2. Security Scan Container Image (`security-scan.yml`)
Scans container images for security vulnerabilities using Trivy.

## Build and Push Workflow

### Triggers
- **Automatic**: On push to `main` or `develop` branches, or when tags starting with `v` are pushed
- **Manual**: Can be manually dispatched with custom parameters
- **Pull Requests**: Builds but doesn't push (for testing)

### Features
✅ **Multi-platform builds**: linux/amd64, linux/arm64  
✅ **Smart tagging**: Automatic tags based on git events  
✅ **Caching**: GitHub Actions cache for faster builds  
✅ **Provenance**: SBOM and provenance metadata  
✅ **Registry push**: Automatic push to GHCR  

### Image Tags Generated
- `latest` - Always points to the latest build
- `main` - Latest build from main branch
- `develop` - Latest build from develop branch
- `v1.0.0` - Semantic version tags
- `v1.0` - Major.minor version tags
- `v1` - Major version tags
- `main-abc123` - Branch with commit SHA

### Manual Dispatch
1. Go to Actions → "Build and Push Container Image"
2. Click "Run workflow"
3. Set custom image tag (optional)
4. Choose whether to push to registry
5. Click "Run workflow"

## Security Scan Workflow

### Triggers
- **Automatic**: Runs after successful build workflow completion
- **Manual**: Can be manually dispatched to scan any image

### Security Features
🔒 **Comprehensive scanning**: OS and library vulnerabilities  
🔒 **Fail-fast**: Fails build on any security warnings by default  
🔒 **Configurable thresholds**: Adjustable severity levels  
🔒 **Detailed reporting**: SARIF format with GitHub integration  
🔒 **Actionable recommendations**: Steps to fix vulnerabilities  

### Severity Levels
- **CRITICAL**: Always fails the build
- **HIGH**: Always fails the build
- **MEDIUM**: Fails if `fail_on_warnings: true`
- **LOW**: Fails if `fail_on_warnings: true`
- **UNKNOWN**: Report all vulnerabilities

### Manual Security Scan
1. Go to Actions → "Security Scan Container Image"
2. Click "Run workflow"
3. Enter image reference to scan
4. Set security thresholds
5. Choose whether to fail on warnings
6. Click "Run workflow"

## Workflow Integration

### Automatic Flow
```
Push to main/develop → Build Image → Push to GHCR → Security Scan
```

### Pull Request Flow
```
PR → Build Image (no push) → Security Scan
```

### Tag Release Flow
```
Push v* tag → Build Image → Push to GHCR → Security Scan
```

## Configuration

### Environment Variables
- `REGISTRY`: Container registry (default: ghcr.io)
- `IMAGE_NAME`: Image name (default: vaulpner)

### Required Secrets
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions

### Registry Authentication
Uses `GITHUB_TOKEN` for GHCR authentication. For other registries, add:
- `REGISTRY_USERNAME`
- `REGISTRY_PASSWORD`

## Usage Examples

### Basic Build
```yaml
# This happens automatically on push to main
# No additional configuration needed
```

### Custom Build with Manual Dispatch
1. Go to Actions tab
2. Select "Build and Push Container Image"
3. Set custom image tag: `my-feature`
4. Enable registry push
5. Run workflow

### Security Scan Only
1. Go to Actions tab
2. Select "Security Scan Container Image"
3. Enter image: `ghcr.io/username/vaulpner:latest`
4. Set severity threshold: `MEDIUM`
5. Enable fail on warnings
6. Run workflow

## Output and Reports

### Build Summary
- Image reference and tags
- Build status and event details
- Multi-platform support info
- Cache and optimization details

### Security Report
- Vulnerability counts by severity
- Overall security status
- Actionable recommendations
- Integration with GitHub Security tab

## Best Practices

### 1. Security
- Always run security scans after builds
- Use `fail_on_warnings: true` for production
- Review and fix vulnerabilities promptly
- Keep base images updated

### 2. Tagging
- Use semantic versioning for releases
- Tag important commits with descriptive names
- Avoid overwriting existing tags

### 3. Registry Management
- Use GHCR for public images
- Consider private registries for sensitive images
- Implement proper access controls

### 4. Monitoring
- Check workflow runs regularly
- Review security scan results
- Monitor build times and cache efficiency

## Troubleshooting

### Build Failures
- Check Dockerfile syntax
- Verify build context
- Review resource limits
- Check registry permissions

### Security Scan Failures
- Review vulnerability details
- Update base images
- Fix vulnerable dependencies
- Consider using `fail_on_warnings: false` temporarily

### Registry Push Issues
- Verify authentication
- Check image size limits
- Ensure registry availability
- Review network connectivity

## Dependencies

### Actions Used
- `actions/checkout@v4` - Code checkout
- `docker/setup-buildx-action@v3` - Docker Buildx setup
- `docker/login-action@v3` - Registry authentication
- `docker/metadata-action@v5` - Image metadata
- `docker/build-push-action@v5` - Build and push
- `aquasecurity/trivy-action@master` - Security scanning
- `github/codeql-action/upload-sarif@v2` - SARIF upload

### Tools
- Docker Buildx for multi-platform builds
- Trivy for vulnerability scanning
- GitHub Actions cache for optimization
- SARIF for security reporting

## Security Considerations

- Workflows run in isolated environments
- Secrets are encrypted and secure
- Minimal permissions required
- Audit trail for all actions
- Vulnerability reporting integration
