# Testing & CI/CD Guide

## Overview

This project includes comprehensive unit tests, integration tests, and automated CI/CD pipelines using GitHub Actions.

## Test Structure

### Unit Tests

Unit tests are located in the same modules they test:

#### Parser Tests (`src/parser/tests.rs`)
- `test_parse_valid_profile`: Tests successful parsing of a complete profile
- `test_parse_education`: Validates education field parsing
- `test_parse_looking_for`: Validates "looking for" preferences parsing
- `test_parse_missing_frontmatter`: Ensures error handling when YAML is missing
- `test_parse_nonexistent_file`: Tests error handling for missing files
- `test_parse_invalid_yaml`: Tests error handling for malformed YAML
- `test_parse_content_extraction`: Validates markdown content extraction
- `test_parse_empty_interests`: Tests handling of empty interest lists
- `test_parse_multiple_interests`: Tests parsing of multiple interests

**Total: 9 parser tests**

#### Models Tests (`src/models/tests.rs`)
- `test_age_range_creation`: Validates AgeRange struct creation
- `test_age_range_validity`: Ensures age range min < max
- `test_education_creation`: Validates Education struct
- `test_looking_for_creation`: Validates LookingFor struct
- `test_profile_details_creation`: Tests complete ProfileDetails creation
- `test_user_profile_creation`: Tests UserProfile wrapper
- `test_empty_interests_list`: Validates empty interest handling
- `test_profile_with_many_interests`: Tests profiles with multiple interests

**Total: 8 model tests**

### Integration Tests

Integration tests are in `tests/integration_tests.rs`:
- `test_parse_complete_profile_workflow`: Full end-to-end parsing workflow
- `test_multiple_profiles_parsing`: Tests parsing multiple profile files

**Total: 2 integration tests**

### Test Coverage

Total test count: **19 tests**
- Unit tests: 17
- Integration tests: 2

All tests use the `tempfile` crate to create isolated temporary file systems.

## Running Tests

### Run all tests
```bash
cargo test
```

### Run only unit tests
```bash
cargo test --lib
```

### Run only integration tests
```bash
cargo test --test integration_tests
```

### Run tests with output
```bash
cargo test -- --nocapture
```

### Run a specific test
```bash
cargo test test_parse_valid_profile
```

### Run tests with coverage (requires tarpaulin)
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Code Quality Checks

### Format Check
```bash
cargo fmt --check
```

### Format Code
```bash
cargo fmt
```

### Lint with Clippy
```bash
cargo clippy --all
```

### Fix Clippy Issues
```bash
cargo clippy --fix --allow-dirty
```

### Security Audit
```bash
cargo install cargo-audit
cargo audit
```

## CI/CD Pipelines

### Continuous Integration (`.github/workflows/ci.yml`)

Runs on every push to `main` and `develop` branches, and on pull requests.

**Jobs:**
1. **Test Suite** - Runs all tests with cargo test
2. **Code Formatting** - Checks code formatting with cargo fmt
3. **Linting** - Runs clippy with warnings as errors
4. **Build** - Builds both debug and release versions
5. **Security Audit** - Scans dependencies for known vulnerabilities

**Caching:** The pipeline caches Cargo registry, index, and build artifacts to improve performance.

### Release Workflow (`.github/workflows/release.yml`)

Triggered when a version tag (v*.*.* format) is pushed.

**Jobs:**
1. **Create Release** - Creates a GitHub release with changelog from commits
2. **Build Release** - Cross-platform builds for:
   - Linux (ubuntu-latest)
   - macOS (macos-latest)
   - Windows (windows-latest)

**Artifacts:** Release binaries are uploaded as release assets for download.

## Local Testing Checklist

Before committing, run this checklist:

- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy --all`
- [ ] Code builds: `cargo build --release`
- [ ] No security issues: `cargo audit`

## Test Dependencies

The following dev-dependencies are used for testing:
- `tempfile`: v3.8 - For creating isolated test file systems

## Continuous Integration Badges

You can add CI badges to your README:

```markdown
![CI](https://github.com/OWNER/REPO/workflows/CI/badge.svg)
![Build Status](https://github.com/OWNER/REPO/workflows/Build/badge.svg)
```

Replace `OWNER/REPO` with your GitHub organization and repository name.

## Troubleshooting

### Tests fail locally but pass in CI
- Ensure you're using the same Rust version: Check `.github/workflows/ci.yml`
- Clear cargo cache: `cargo clean`
- Rebuild: `cargo test`

### Clippy warnings in CI
- Run locally: `cargo clippy --all`
- Fix: `cargo clippy --fix --allow-dirty`
- Format: `cargo fmt`

### Performance issues with large test suites
- Run tests in parallel: `cargo test -- --test-threads=4`
- Use release mode for tests: `cargo test --release`

## Adding New Tests

When adding new features:

1. Write tests first (TDD approach)
2. Place unit tests in the module being tested
3. Place integration tests in `tests/integration_tests.rs`
4. Use descriptive test names: `test_feature_behavior`
5. Use `tempfile::TempDir` for file operations
6. Run full test suite before committing: `cargo test --all`

## Test Best Practices

1. **Isolation**: Each test is independent and uses temp directories
2. **Descriptive names**: Test names clearly describe what they test
3. **Given-When-Then**: Tests follow the arrange-act-assert pattern
4. **Error cases**: Tests include both success and failure paths
5. **Content validation**: Tests verify both structure and content
6. **No flakiness**: Tests are deterministic and always pass/fail consistently

## Version Management

Current version: **0.1.0** (from `Cargo.toml`)

When releasing:
1. Update version in `Cargo.toml`
2. Ensure all tests pass
3. Create git tag: `git tag -a v0.2.0 -m "Version 0.2.0"`
4. Push tag: `git push origin v0.2.0`
5. GitHub Actions will automatically create a release with binaries
