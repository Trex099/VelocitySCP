# Security Policy

## Security Design Principles

Velocity Console is designed with security as a first-class concern:

### Credential Handling

- **No plaintext passwords in storage** - All credentials are stored in the OS keyring (Secret Service on Linux)
- **Database stores references only** - SQLite stores UUIDs that reference keyring entries, never the passwords themselves
- **No credentials in logs** - `SecretString` wrapper automatically redacts credentials in Debug/Display output
- **Memory zeroization** - Sensitive data is zeroed from memory when dropped

### Input Validation

All user-provided inputs are validated before use:

- **Hostnames** - Checked for command injection characters, null bytes, path separators
- **Paths** - Checked for path traversal sequences (`..`)
- **Ports** - Validated to be in valid TCP range (1-65535)

### SSH Security

- **Host key verification** - Known hosts are verified to prevent MITM attacks
- **Encrypted channels** - All data flows through SSH encryption
- **No shell command injection** - User inputs are validated before being used in commands

## Reporting Vulnerabilities

If you discover a security vulnerability, please:

1. **Do NOT open a public issue**
2. Email the maintainers directly with details
3. Allow up to 48 hours for initial response

## Security Checklist for Contributors

- [ ] No hardcoded secrets or credentials
- [ ] Use `SecretString` for any sensitive data
- [ ] Validate all user inputs
- [ ] No sensitive data in log messages
- [ ] No eval() or dynamic code execution with user input
