# Contributing to vaulpner

Thank you for your interest in contributing to vaulpner! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

### Reporting Issues

Before creating an issue, please:
1. Check if the issue already exists
2. Search through closed issues
3. Verify you're using the latest version

When creating an issue, please include:
- **Description**: Clear description of the problem
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: OS, Kubernetes version, Vault version
- **Logs**: Relevant log output (with sensitive data redacted)

### Suggesting Enhancements

For feature requests, please include:
- **Use Case**: Describe the problem you're trying to solve
- **Proposed Solution**: Your idea for solving the problem
- **Alternatives**: Other solutions you've considered
- **Additional Context**: Any other relevant information

## 🛠️ Development Setup

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Cargo
- Docker (for container testing)
- Kubernetes cluster (for integration testing)
- HashiCorp Vault

### Building from Source

```bash
# Clone the repository
git clone https://github.com/outsideorbit/vaulpner.git
cd vaulpner

# Install dependencies
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy --all-targets -- -D warnings

# Format code
cargo fmt
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_tests
```

### Building Container Image

```bash
# Build container image
docker build -t vaulpner:latest .

# Build multi-platform image
docker buildx build --platform linux/amd64,linux/arm64 -t vaulpner:latest .
```

## 📝 Code Style

### Rust Conventions

- Follow standard Rust naming conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Write comprehensive tests for new functionality
- Document public APIs with doc comments

### Commit Messages

We follow conventional commit format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat: add health check endpoint
fix: resolve memory leak in token handling
docs: update deployment examples
test: add integration tests for vault initialization
```

## 🧪 Testing

### Unit Tests

- Write unit tests for all public functions
- Use descriptive test names
- Test both success and error cases
- Mock external dependencies when possible

### Integration Tests

- Test complete workflows
- Use real Vault instances in test containers
- Test error scenarios and edge cases
- Verify Kubernetes secret creation

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_function_success() {
        // Test successful case
    }

    #[tokio::test]
    async fn test_function_error() {
        // Test error case
    }
}
```

## 📋 Pull Request Process

### Before Submitting

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Make** your changes
4. **Add** tests for new functionality
5. **Update** documentation if needed
6. **Run** all tests and checks
7. **Commit** your changes with conventional commit messages

### Pull Request Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Commit messages follow conventional format
- [ ] All CI checks pass
- [ ] No merge conflicts

### Review Process

1. **Automated Checks**: CI will run tests, linting, and security checks
2. **Code Review**: Maintainers will review your code
3. **Feedback**: Address any feedback or requested changes
4. **Approval**: Once approved, your PR will be merged

## 🐛 Debugging

### Common Issues

#### Build Failures
```bash
# Clean build
cargo clean
cargo build

# Check Rust version
rustc --version
cargo --version
```

#### Test Failures
```bash
# Run tests with more output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture
```

#### Container Build Issues
```bash
# Check Docker build context
docker build --no-cache -t vaulpner:latest .

# Check .dockerignore
cat .dockerignore
```

## 📚 Documentation

### Code Documentation

- Document all public APIs
- Use doc comments with examples
- Include error conditions
- Document configuration options

### User Documentation

- Update README.md for user-facing changes
- Add examples for new features
- Update troubleshooting section
- Keep installation instructions current

## 🔒 Security

### Security Considerations

- Never commit secrets or credentials
- Use environment variables for configuration
- Validate all inputs
- Handle errors gracefully
- Follow principle of least privilege

### Reporting Security Issues

For security-related issues, please:
1. **Do not** create a public issue
2. Email security@outsideorbit.com
3. Include detailed description
4. Wait for response before public disclosure

## 🏷️ Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] Version bumped in Cargo.toml
- [ ] Changelog updated
- [ ] Release notes prepared
- [ ] Container image built and pushed

## 🤔 Questions?

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Create an issue for bugs or feature requests
- **Email**: Contact maintainers directly for sensitive matters

## 📄 License

By contributing to vaulpner, you agree that your contributions will be licensed under the MIT License.
