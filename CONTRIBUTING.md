# Contributing to DeepSeek-Rust

Thank you for your interest in contributing to DeepSeek-Rust! We welcome contributions from everyone.

## Code of Conduct

Please note that this project adheres to a Code of Conduct. By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, please include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, etc.)
- Any relevant logs or error messages

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- A clear and descriptive title
- A detailed description of the proposed enhancement
- Use cases for the enhancement
- Any possible drawbacks or alternatives

### Pull Requests

1. **Fork the repository** and create your branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Set up your development environment**:
   ```bash
   # Clone your fork
   git clone https://github.com/your-username/deepseek-rust.git
   cd deepseek-rust
   
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install development tools
   rustup component add rustfmt clippy
   cargo install cargo-tarpaulin cargo-audit
   ```

3. **Make your changes**:
   - Write clear, idiomatic Rust code
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation as needed

4. **Ensure code quality**:
   ```bash
   # Format your code
   cargo fmt
   
   # Run clippy
   cargo clippy --all-targets --all-features -- -D warnings
   
   # Run tests
   cargo test --all-features
   
   # Check documentation
   cargo doc --no-deps --all-features
   ```

5. **Commit your changes**:
   - Use clear and meaningful commit messages
   - Follow conventional commit format: `type(scope): description`
   - Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
   - Example: `feat(client): add streaming support`

6. **Push to your fork** and submit a pull request:
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Pull Request Guidelines**:
   - Provide a clear description of the changes
   - Reference any related issues
   - Ensure all CI checks pass
   - Be responsive to review feedback

## Development Guidelines

### Code Style

- Follow Rust naming conventions
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Prefer explicit types over type inference in public APIs
- Document all public items with doc comments

### Testing

- Write unit tests for new functions
- Write integration tests for new features
- Aim for high test coverage (>80%)
- Use meaningful test names that describe what is being tested

### Documentation

- Add doc comments to all public items
- Include examples in doc comments
- Update README.md if adding new features
- Keep CHANGELOG.md updated

### Performance

- Avoid unnecessary allocations
- Use `&str` instead of `String` where possible
- Consider using `Cow<str>` for flexible string handling
- Profile performance-critical code

## Project Structure

```
deepseek-rust/
├── src/
│   ├── lib.rs          # Library entry point
│   ├── client.rs       # Main client implementation
│   ├── config.rs       # Configuration
│   ├── error.rs        # Error types
│   └── models/         # Request/Response types
├── examples/           # Example usage
├── tests/              # Integration tests
└── benches/            # Benchmarks
```

## Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Examples:
- `feat(client): add retry logic for failed requests`
- `fix(models): correct temperature validation range`
- `docs(readme): update installation instructions`
- `test(client): add tests for error handling`

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create a git tag: `git tag v0.1.0`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions will handle the rest

## Getting Help

- Open an issue for bugs or feature requests
- Join discussions in GitHub Discussions
- Check existing documentation and examples

## Recognition

Contributors will be recognized in:
- The project README
- Release notes
- GitHub contributors page

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache 2.0 licenses.

Thank you for contributing to DeepSeek-Rust! 🦀