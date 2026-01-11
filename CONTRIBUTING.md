# Contributing to Velocity Console

## Development Workflow

1. **Create a feature branch** from `develop`
   ```bash
   git checkout develop
   git checkout -b feature/your-feature-name
   ```

2. **Write tests first** (TDD)
   - Rust unit tests in `src-tauri/src/`
   - Integration tests in `src-tauri/tests/`

3. **Implement the feature**

4. **Ensure all checks pass**
   ```bash
   # Rust
   cd src-tauri
   cargo fmt --all
   cargo clippy --all-targets --all-features
   cargo test --lib

   # Frontend
   npm run lint
   npm run format
   npm run build
   ```

5. **Submit a PR** to `develop`

## Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add server grouping
fix: correct SSH timeout handling
docs: update README
test: add connection pool tests
refactor: simplify error handling
chore: update dependencies
```

## Code Style

### Rust

- Follow clippy pedantic lints
- Use `SecretString` for sensitive data
- Add doc comments to public functions
- Include `# Errors` section for fallible functions

### TypeScript

- No `any` types
- Explicit return types for functions
- Use single quotes
- 2-space indentation

## Security Requirements

- Never expose credentials in logs
- Validate all user inputs
- Use `SecretString` for passwords
- See `/docs/security/SECURITY.md`
