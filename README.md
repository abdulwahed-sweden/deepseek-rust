# 🦀 DeepSeek-Rust

A powerful, async Rust client library for the DeepSeek AI API with full type safety and comprehensive error handling.

[![Crates.io](https://img.shields.io/crates/v/deepseek-rust.svg)](https://crates.io/crates/deepseek-rust)
[![Documentation](https://docs.rs/deepseek-rust/badge.svg)](https://docs.rs/deepseek-rust)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE)
[![Build Status](https://github.com/abdulwahed-sweden/deepseek-rust/workflows/CI/badge.svg)](https://github.com/abdulwahed-sweden/deepseek-rust/actions)

## ✨ Features

- 🚀 **Fully Async** - Built on Tokio for high-performance async operations
- 🔒 **Type Safe** - Leverage Rust's type system for compile-time safety
- 🔄 **Automatic Retries** - Built-in exponential backoff for transient failures
- 🧠 **Multiple Models** - Support for Chat, Reasoner, and Coder models
- 🏗️ **Builder Pattern** - Intuitive API with method chaining
- 📊 **Token Tracking** - Monitor usage for cost management
- 🔐 **Secure** - API keys handled securely with the `secrecy` crate
- 📝 **Comprehensive Logging** - Built-in tracing for debugging

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
deepseek-rust = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## 🚀 Quick Start

```rust
use deepseek_rust::{DeepSeekClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client from environment variable DEEPSEEK_API_KEY
    let client = DeepSeekClient::from_env()?;
    
    // Send a simple message
    let response = client
        .chat()
        .add_user_message("Hello, DeepSeek!")
        .send()
        .await?;
    
    println!("{}", response.choices[0].message.content.as_ref().unwrap());
    Ok(())
}
```

## 🔧 Configuration

### Environment Variables

Create a `.env` file in your project root:

```env
DEEPSEEK_API_KEY=your_api_key_here
DEEPSEEK_API_BASE_URL=https://api.deepseek.com  # Optional
DEEPSEEK_TIMEOUT_SECONDS=30                     # Optional
```

### Programmatic Configuration

```rust
use deepseek_rust::{DeepSeekClient, DeepSeekConfig};
use std::time::Duration;

let config = DeepSeekConfig::new("your-api-key")
    .with_base_url("https://api.deepseek.com")
    .with_timeout(Duration::from_secs(60));

let client = DeepSeekClient::new(config)?;
```

## 📚 Examples

### Basic Chat

```rust
let response = client
    .chat()
    .add_user_message("Explain quantum computing")
    .send()
    .await?;
```

### With System Message

```rust
let response = client
    .chat()
    .add_system_message("You are a helpful coding assistant")
    .add_user_message("Write a function to reverse a string in Rust")
    .send()
    .await?;
```

### Using Reasoning Model

```rust
let response = client
    .chat()
    .add_user_message("What is 15 * 47? Show your work.")
    .with_model(Model::Reasoner)
    .send()
    .await?;

// Access reasoning content
if let Some(reasoning) = &response.choices[0].message.reasoning_content {
    println!("Reasoning: {}", reasoning);
}
```

### Multi-turn Conversation

```rust
let response = client
    .chat()
    .add_user_message("My name is Alice")
    .add_assistant_message("Hello Alice! How can I help you?")
    .add_user_message("What's my name?")
    .send()
    .await?;
```

### With Parameters

```rust
let response = client
    .chat()
    .add_user_message("Write a creative story")
    .with_temperature(0.9)?  // Higher = more creative
    .with_max_tokens(500)    // Limit response length
    .send()
    .await?;
```

## 🏗️ Advanced Usage

### Custom Error Handling

```rust
use deepseek_rust::{DeepSeekError, Result};

match client.chat().add_user_message("Hello").send().await {
    Ok(response) => println!("Success!"),
    Err(DeepSeekError::RateLimitExceeded) => {
        println!("Rate limited, please wait...");
    }
    Err(DeepSeekError::ApiError { status, message }) => {
        println!("API error {}: {}", status, message);
    }
    Err(e) => println!("Other error: {}", e),
}
```

### Token Usage Tracking

```rust
let response = client.chat()
    .add_user_message("Hello")
    .send()
    .await?;

if let Some(usage) = &response.usage {
    println!("Tokens used:");
    println!("  Prompt: {}", usage.prompt_tokens);
    println!("  Completion: {}", usage.completion_tokens);
    println!("  Total: {}", usage.total_tokens);
}
```

### Connection Testing

```rust
// Test API connectivity
client.test_connection().await?;
```

## 🧪 Running Examples

```bash
# Clone the repository
git clone https://github.com/abdulwahed-sweden/deepseek-rust.git
cd deepseek-rust

# Set your API key
echo "DEEPSEEK_API_KEY=your_key_here" > .env

# Run the basic example
cargo run --example basic

# Run with logging
RUST_LOG=deepseek_rust=debug cargo run --example basic
```

## 🛠️ Development

### Project Structure

```
deepseek-rust/
├── src/
│   ├── lib.rs          # Library entry point
│   ├── client.rs       # Main client implementation
│   ├── config.rs       # Configuration
│   ├── error.rs        # Error types
│   └── models/         # Request/Response types
│       ├── request.rs
│       └── response.rs
├── examples/
│   └── basic.rs        # Usage examples
└── tests/
    └── integration.rs  # Integration tests
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture

# Run specific test
cargo test test_chat_builder
```

### Building Documentation

```bash
# Build and open documentation
cargo doc --open
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- Built with ❤️ by [Abdulwahed](https://github.com/abdulwahed-sweden)
- Inspired by the DeepSeek AI platform
- Thanks to the Rust community for amazing crates

## 📮 Contact

- **GitHub**: [@abdulwahed-sweden](https://github.com/abdulwahed-sweden)
- **Issues**: [Report bugs or request features](https://github.com/abdulwahed-sweden/deepseek-rust/issues)

## 🚧 Roadmap

- [x] Basic chat completions
- [x] Multiple model support
- [x] Automatic retry logic
- [ ] Streaming responses
- [ ] File uploads
- [ ] Function calling
- [ ] Token counting before requests
- [ ] Response caching
- [ ] Rate limit handling with queues
- [ ] WebAssembly support

---

**Note**: This is an unofficial client library. For official DeepSeek documentation, visit [deepseek.com](https://deepseek.com).
