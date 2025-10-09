# GitHub Workflows

This directory contains GitHub workflows for building and pushing container images.

## Workflows Overview

### 1. Build and Push Container Image (`build-and-push.yml`)
A callable workflow that builds the vaulpner container image and pushes it to GitHub Container Registry (GHCR). This workflow can be invoked by other workflows or run manually with custom parameters.

## Build and Push Workflow

### Triggers
- **Workflow Call**: Can be invoked by other workflows
- **Manual**: Can be manually dispatched with custom parameters

### Features
✅ **Multi-platform builds**: linux/amd64, linux/arm64  
✅ **Configurable builds**: Production and debug images  
✅ **Provenance**: SBOM and provenance metadata  
✅ **Registry push**: Configurable push to GHCR  
✅ **Callable**: Reusable workflow with inputs  

### Workflow Inputs
- `registry` - Container registry URL (default: ghcr.io)
- `image_name` - Image name (default: vaulpner)
- `production_tag` - Production image tag (default: latest)
- `debug_tag` - Debug image tag (default: debug)
- `push_images` - Whether to push images (default: true)
- `build_production` - Whether to build production image (default: true)
- `build_debug` - Whether to build debug image (default: true)

### Image Tags Generated
- **Production**: `ghcr.io/owner/vaulpner:latest`
- **Debug**: `ghcr.io/owner/vaulpner:debug`

### Manual Dispatch
1. Go to Actions → "Build and Push Container Image"
2. Click "Run workflow"
3. Set custom parameters (registry, image name, tags, etc.)
4. Choose which images to build
5. Click "Run workflow"

## Container Images

### Production Image
- **Base**: Distroless (minimal, secure)
- **Binary**: Release build (optimized)
- **Size**: Minimal attack surface
- **Purpose**: Production deployments

### Debug Image
- **Base**: Ubuntu 22.04
- **Binary**: Debug build (with symbols)
- **Tools**: gdb, valgrind, strace, etc.
- **Source**: Included for debugging
- **Purpose**: Development and troubleshooting

## Workflow Integration

### Callable Workflow Usage
```yaml
# Example: Calling from another workflow
jobs:
  build:
    uses: ./.github/workflows/build-and-push.yml
    with:
      registry: 'ghcr.io'
      image_name: 'vaulpner'
      production_tag: 'latest'
      debug_tag: 'debug'
      push_images: true
      build_production: true
      build_debug: true
```

### Manual Execution
```
Manual Trigger → Build Images → Push to GHCR → Generate Summary
```

## Configuration

### Workflow Inputs
- `registry`: Container registry URL (default: ghcr.io)
- `image_name`: Image name (default: vaulpner)
- `production_tag`: Production image tag (default: latest)
- `debug_tag`: Debug image tag (default: debug)
- `push_images`: Whether to push images (default: true)
- `build_production`: Whether to build production image (default: true)
- `build_debug`: Whether to build debug image (default: true)

### Required Secrets
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions

### Registry Authentication
Uses `GITHUB_TOKEN` for GHCR authentication. For other registries, add:
- `REGISTRY_USERNAME`
- `REGISTRY_PASSWORD`

## Usage Examples

### Basic Build (Manual)
1. Go to Actions tab
2. Select "Build and Push Container Image"
3. Use default parameters
4. Run workflow

### Custom Build with Manual Dispatch
1. Go to Actions tab
2. Select "Build and Push Container Image"
3. Set custom parameters:
   - `production_tag`: `v1.0.0`
   - `debug_tag`: `debug-v1.0.0`
   - `push_images`: `true`
4. Run workflow

### Callable Workflow Usage
```yaml
# In another workflow file
name: CI/CD Pipeline
on: [push, pull_request]

jobs:
  build-and-push:
    uses: ./.github/workflows/build-and-push.yml
    with:
      registry: 'ghcr.io'
      image_name: 'vaulpner'
      production_tag: 'latest'
      debug_tag: 'debug'
      push_images: true
      build_production: true
      build_debug: true
```

## Output and Reports

### Build Summary
- Image reference and tags
- Build status and event details
- Multi-platform support info
- Production and debug image details
- Registry push status

## Best Practices

### 1. Container Images
- Use production image for deployments
- Use debug image for troubleshooting
- Keep base images updated
- Test both images before deployment

### 2. Tagging
- Use descriptive tags for releases
- Tag important commits with meaningful names
- Avoid overwriting existing tags
- Use consistent naming conventions

### 3. Registry Management
- Use GHCR for public images
- Consider private registries for sensitive images
- Implement proper access controls
- Monitor registry usage

### 4. Workflow Usage
- Use callable workflow for CI/CD pipelines
- Set appropriate input parameters
- Monitor workflow runs regularly
- Review build summaries

## Troubleshooting

### Build Failures
- Check Containerfile syntax
- Verify build context
- Review resource limits
- Check registry permissions
- Ensure all required inputs are provided

### Registry Push Issues
- Verify authentication
- Check image size limits
- Ensure registry availability
- Review network connectivity
- Verify `push_images` input is set to `true`

### Workflow Call Issues
- Check input parameter types
- Verify required inputs are provided
- Review workflow permissions
- Check for syntax errors in calling workflow

## Dependencies

### Actions Used
- `actions/checkout@v4` - Code checkout
- `docker/setup-buildx-action@v3` - Docker Buildx setup
- `docker/login-action@v3` - Registry authentication
- `docker/build-push-action@v6` - Build and push

### Tools
- Docker Buildx for multi-platform builds
- GitHub Actions cache for optimization
- Multi-stage container builds

## Security Considerations

- Workflows run in isolated environments
- Secrets are encrypted and secure
- Minimal permissions required
- Audit trail for all actions
- Multi-platform builds with provenance
- SBOM generation for supply chain security
