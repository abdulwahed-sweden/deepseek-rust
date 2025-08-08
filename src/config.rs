//! Configuration module for DeepSeek API client

use crate::error::{DeepSeekError, Result};
use secrecy::{ExposeSecret, Secret};
use std::time::Duration;

/// Default API base URL
pub const DEFAULT_BASE_URL: &str = "https://api.deepseek.com";

/// Default timeout in seconds
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default max retries
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Configuration for the DeepSeek API client
#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    /// API key for authentication
    pub api_key: Secret<String>,
    
    /// Base URL for the API
    pub base_url: String,
    
    /// Request timeout duration
    pub timeout: Duration,
    
    /// Maximum number of retries for failed requests
    pub max_retries: u32,
    
    /// Whether to validate SSL certificates (should be true in production)
    pub validate_certs: bool,
    
    /// Optional proxy URL
    pub proxy: Option<String>,
    
    /// User agent string
    pub user_agent: String,
}

impl DeepSeekConfig {
    /// Create a new configuration with an API key
    /// 
    /// # Example
    /// ```
    /// use deepseek_rust::DeepSeekConfig;
    /// 
    /// let config = DeepSeekConfig::new("your-api-key");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: Secret::new(api_key.into()),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_retries: DEFAULT_MAX_RETRIES,
            validate_certs: true,
            proxy: None,
            user_agent: format!("deepseek-rust/{}", env!("CARGO_PKG_VERSION")),
        }
    }
    
    /// Create configuration from environment variables
    /// 
    /// Looks for:
    /// - `DEEPSEEK_API_KEY` (required)
    /// - `DEEPSEEK_API_BASE_URL` (optional)
    /// - `DEEPSEEK_TIMEOUT_SECONDS` (optional)
    /// - `DEEPSEEK_MAX_RETRIES` (optional)
    /// - `DEEPSEEK_PROXY` (optional)
    /// 
    /// # Example
    /// ```no_run
    /// use deepseek_rust::DeepSeekConfig;
    /// 
    /// let config = DeepSeekConfig::from_env().expect("Failed to load config");
    /// ```
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists (ignore errors if it doesn't)
        dotenvy::dotenv().ok();
        
        // Get required API key
        let api_key = std::env::var("DEEPSEEK_API_KEY")
            .map_err(|_| DeepSeekError::ConfigError(
                "DEEPSEEK_API_KEY environment variable not found. \
                Please set it to your DeepSeek API key.".to_string()
            ))?;
        
        // Validate API key format
        if api_key.trim().is_empty() {
            return Err(DeepSeekError::ConfigError(
                "DEEPSEEK_API_KEY cannot be empty".to_string()
            ));
        }
        
        // Get optional base URL
        let base_url = std::env::var("DEEPSEEK_API_BASE_URL")
            .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
        
        // Get optional timeout
        let timeout_secs = std::env::var("DEEPSEEK_TIMEOUT_SECONDS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_TIMEOUT_SECS);
        
        // Get optional max retries
        let max_retries = std::env::var("DEEPSEEK_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(DEFAULT_MAX_RETRIES);
        
        // Get optional proxy
        let proxy = std::env::var("DEEPSEEK_PROXY").ok();
        
        // Get optional cert validation setting
        let validate_certs = std::env::var("DEEPSEEK_VALIDATE_CERTS")
            .ok()
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(true);
        
        Ok(Self {
            api_key: Secret::new(api_key),
            base_url,
            timeout: Duration::from_secs(timeout_secs),
            max_retries,
            validate_certs,
            proxy,
            user_agent: format!("deepseek-rust/{}", env!("CARGO_PKG_VERSION")),
        })
    }
    
    /// Set the base URL
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
    
    /// Set the timeout duration
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// Set the maximum number of retries
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
    
    /// Set proxy URL
    pub fn with_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }
    
    /// Set whether to validate SSL certificates
    pub fn with_validate_certs(mut self, validate: bool) -> Self {
        self.validate_certs = validate;
        self
    }
    
    /// Set custom user agent
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Check API key
        if self.api_key.expose_secret().trim().is_empty() {
            return Err(DeepSeekError::ConfigError(
                "API key cannot be empty".to_string()
            ));
        }
        
        // Check base URL
        if self.base_url.trim().is_empty() {
            return Err(DeepSeekError::ConfigError(
                "Base URL cannot be empty".to_string()
            ));
        }
        
        // Validate URL format
        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err(DeepSeekError::ConfigError(
                "Base URL must start with http:// or https://".to_string()
            ));
        }
        
        // Check timeout
        if self.timeout.as_secs() == 0 {
            return Err(DeepSeekError::ConfigError(
                "Timeout must be greater than 0".to_string()
            ));
        }
        
        Ok(())
    }
}

impl Default for DeepSeekConfig {
    /// Create a default configuration (requires API key to be set later)
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_new() {
        let config = DeepSeekConfig::new("test-key");
        assert_eq!(config.api_key.expose_secret(), "test-key");
        assert_eq!(config.base_url, DEFAULT_BASE_URL);
        assert_eq!(config.timeout, Duration::from_secs(DEFAULT_TIMEOUT_SECS));
        assert_eq!(config.max_retries, DEFAULT_MAX_RETRIES);
    }
    
    #[test]
    fn test_config_builder() {
        let config = DeepSeekConfig::new("test-key")
            .with_base_url("https://custom.api.com")
            .with_timeout(Duration::from_secs(60))
            .with_max_retries(5)
            .with_proxy("http://proxy.example.com")
            .with_validate_certs(false);
        
        assert_eq!(config.base_url, "https://custom.api.com");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.proxy, Some("http://proxy.example.com".to_string()));
        assert!(!config.validate_certs);
    }
    
    #[test]
    fn test_config_validation() {
        // Valid config
        let valid_config = DeepSeekConfig::new("test-key");
        assert!(valid_config.validate().is_ok());
        
        // Empty API key
        let empty_key_config = DeepSeekConfig::new("");
        assert!(empty_key_config.validate().is_err());
        
        // Invalid URL
        let invalid_url_config = DeepSeekConfig::new("test-key")
            .with_base_url("not-a-url");
        assert!(invalid_url_config.validate().is_err());
        
        // Zero timeout
        let zero_timeout_config = DeepSeekConfig::new("test-key")
            .with_timeout(Duration::from_secs(0));
        assert!(zero_timeout_config.validate().is_err());
    }
}