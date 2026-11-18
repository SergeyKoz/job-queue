## ✅ Rust PR Checklist

**Code Quality & Compilation**

- [ ] Code is formatted (`cargo fmt --all -- --check`)
- [ ] No linter warnings (`cargo clippy --all-targets --all-features -- -D warnings`)
- [ ] Project compiles with different feature flag combinations (`cargo check --no-default-features`)
- [ ] No new compiler warnings (dead code, unused variables, etc.)

**Safety & Error Handling**

- [ ] No new `.unwrap()` or `.expect()` calls in production code unless strictly justified
- [ ] Every `unsafe` block includes a `// SAFETY:` comment explaining the invariants
- [ ] New dependencies in `Cargo.toml` are checked for vulnerabilities and licenses (`cargo audit` / `cargo deny`)

**Testing & Documentation**

- [ ] All existing tests pass (`cargo test --all-features`)
- [ ] Unit or integration tests are added for new logic and edge cases
- [ ] Public structs, methods, and modules have up-to-date doc comments (`///`)
- [ ] Documentation and doc-examples compile and pass (`cargo test --doc`)
- [ ] Performance-sensitive changes are verified with benchmarks (`cargo bench`)