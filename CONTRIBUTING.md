# Contributing to Rust Container System

Thank you for your interest in contributing to the Rust Container System! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)

---

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and constructive in all interactions.

### Our Standards

- Be welcoming and inclusive
- Be respectful of differing viewpoints
- Accept constructive criticism gracefully
- Focus on what's best for the community

---

## Getting Started

### Prerequisites

- Rust 1.90.0 or later
- Cargo (comes with Rust)
- Git

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/kcenon/rust_container_system.git
cd rust_container_system

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

---

## Development Setup

### IDE Recommendations

**VS Code** with extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Even Better TOML

**IntelliJ IDEA** with:
- Rust plugin

### Useful Commands

```bash
# Build
cargo build
cargo build --release

# Test
cargo test
cargo test -- --nocapture  # Show output
cargo test --test integration_tests  # Specific test file

# Check code quality
cargo clippy
cargo fmt --check

# Generate documentation
cargo doc --open

# Run benchmarks
cargo bench

# Check for security vulnerabilities
cargo audit
```

---

## How to Contribute

### Types of Contributions

| Type | Description | Label |
|------|-------------|-------|
| Bug fix | Fix incorrect behavior | `bug` |
| Feature | Add new functionality | `enhancement` |
| Documentation | Improve docs | `documentation` |
| Performance | Optimize speed/memory | `performance` |
| Test | Add/improve tests | `testing` |
| Refactor | Improve code structure | `refactor` |

### Contribution Workflow

1. **Find or create an issue**
   - Check existing issues first
   - Create a new issue for discussion

2. **Fork and branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/bug-description
   ```

3. **Make changes**
   - Write code
   - Add tests
   - Update documentation

4. **Verify quality**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

5. **Submit pull request**
   - Fill out the PR template
   - Link related issues

---

## Coding Standards

### Rust Style

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) and use `rustfmt`:

```bash
cargo fmt
```

### Naming Conventions

```rust
// Types: PascalCase
struct ValueContainer { ... }
enum ValueType { ... }
trait Value { ... }

// Functions and methods: snake_case
fn get_value(&self, name: &str) -> Option<...>
fn serialize_cpp_wire(&self) -> Result<String>

// Constants: SCREAMING_SNAKE_CASE
const DEFAULT_MAX_VALUES: usize = 10_000;

// Variables: snake_case
let container = ValueContainer::new();
let value_count = container.value_count();
```

### Documentation

Document all public items:

```rust
/// Brief one-line description.
///
/// More detailed description if needed.
///
/// # Arguments
///
/// * `name` - The name of the value
/// * `value` - The integer value to store
///
/// # Returns
///
/// A new IntValue instance.
///
/// # Examples
///
/// ```
/// use rust_container_system::values::IntValue;
///
/// let value = IntValue::new("count", 42);
/// assert_eq!(value.to_int().unwrap(), 42);
/// ```
pub fn new(name: impl Into<String>, value: i32) -> Self {
    // ...
}
```

### Error Handling

- Use `Result<T, ContainerError>` for fallible operations
- Provide meaningful error messages
- Don't panic in library code

```rust
// Good
pub fn parse(data: &str) -> Result<Self> {
    if data.is_empty() {
        return Err(ContainerError::InvalidDataFormat(
            "Input cannot be empty".to_string()
        ));
    }
    // ...
}

// Bad
pub fn parse(data: &str) -> Self {
    assert!(!data.is_empty()); // Don't panic
    // ...
}
```

### Safety

- No `unsafe` code unless absolutely necessary
- If `unsafe` is needed, document why and ensure soundness
- All `unsafe` blocks require code review

---

## Testing Requirements

### Test Coverage

All contributions must include tests:

| Change Type | Required Tests |
|-------------|----------------|
| Bug fix | Test that reproduces and verifies fix |
| New feature | Unit tests + integration test |
| API change | Update existing tests + new tests |
| Performance | Benchmark before/after |

### Running Tests

```bash
# All tests
cargo test

# With coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html

# Property tests
cargo test --test property_tests

# Benchmarks
cargo bench
```

### Test Quality

- Tests should be independent
- Use descriptive test names
- Test both success and failure cases
- Include edge cases

See [Testing Guide](docs/contributing/TESTING.md) for detailed information.

---

## Pull Request Process

### Before Submitting

1. **All tests pass**
   ```bash
   cargo test --all-features
   ```

2. **No clippy warnings**
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Code is formatted**
   ```bash
   cargo fmt -- --check
   ```

4. **Documentation is updated**
   - Update relevant docs
   - Add examples if needed

5. **CHANGELOG is updated** (for significant changes)

### PR Template

```markdown
## Description
Brief description of changes.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All tests passing

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings
```

### Review Process

1. **Automated checks** must pass
2. **Code review** by maintainer
3. **Changes requested** → Update PR
4. **Approved** → Merge

### Merge Requirements

- All CI checks pass
- At least one approval
- No unresolved conversations
- Up-to-date with main branch

---

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag
4. Publish to crates.io

---

## Getting Help

### Resources

- [Documentation](docs/README.md)
- [FAQ](docs/guides/FAQ.md)
- [Troubleshooting](docs/guides/TROUBLESHOOTING.md)

### Contact

- **Issues**: [GitHub Issues](https://github.com/kcenon/rust_container_system/issues)
- **Discussions**: [GitHub Discussions](https://github.com/kcenon/rust_container_system/discussions)

---

## Recognition

Contributors are recognized in:
- `CHANGELOG.md` for significant contributions
- GitHub contributors page

Thank you for contributing!

---

*Last updated: 2025-11-26*
