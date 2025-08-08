//! # DeepSeek-Rust
//! 
//! A powerful, async Rust client library for the DeepSeek AI API.
//! 
//! ## Quick Start
//! 
//! ```no_run
//! use deepseek_rust::{DeepSeekClient, Message, Model, Result};
//! 
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Create client from environment variables
//!     let client = DeepSeekClient::from_env()?;
//!     
//!     // Send a simple message
//!     let response = client
//!         .chat()
//!         .add_user_message("Hello, DeepSeek!")
//!         .send()
//!         .await?;
//!     
//!     if let Some(content) = &response.choices[0].message.content {
//!         println!("Response: {}", content);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Features
//! 
//! - **Async/Await** - Built on Tokio for high-performance async operations
//! - **Type Safety** - Leverage Rust's type system for compile-time safety
//! - **Multiple Models** - Support for Chat, Reasoner, and Coder models
//! - **Builder Pattern** - Intuitive API with method chaining
//! - **Automatic Retries** - Built-in exponential backoff for transient failures
//! - **Secure** - API keys handled securely with the `secrecy` crate

#![doc(html_logo_url = "https://raw.githubusercontent.com/abdulwahed-sweden/deepseek-rust/main/logo.png")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/abdulwahed-sweden/deepseek-rust/main/favicon.ico")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod client;
pub mod config;
pub mod error;
pub mod models;

// Re-export main types for convenience
pub use client::{ChatBuilder, DeepSeekClient};
pub use config::DeepSeekConfig;
pub use error::{DeepSeekError, Result};

// Re-export model types
pub use models::request::{
    ChatCompletionRequest, Message, Model, Role, Temperature,
};
pub use models::response::{
    ChatCompletionResponse, Choice, ResponseMessage, Usage,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Library authors
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(VERSION, "0.1.0");
    }

    #[test]
    fn test_library_name() {
        assert_eq!(NAME, "deepseek-rust");
    }
}