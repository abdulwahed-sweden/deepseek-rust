//! Error types for the DeepSeek Rust client library

use thiserror::Error;

/// Main error type for DeepSeek API operations
#[derive(Error, Debug)]
pub enum DeepSeekError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    /// JSON parsing failed
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
    
    /// API returned an error
    #[error("API error (status {status}): {message}")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message from API
        message: String,
    },
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// Invalid parameter provided
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded. Please wait before making more requests.")]
    RateLimitExceeded,
    
    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    /// Environment variable error
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Timeout error
    #[error("Request timed out after {0} seconds")]
    TimeoutError(u64),
    
    /// Empty response
    #[error("Received empty response from API")]
    EmptyResponse,
    
    /// Unsupported feature
    #[error("Feature not yet supported: {0}")]
    UnsupportedFeature(String),
}

/// Type alias for Results with DeepSeekError
pub type Result<T> = std::result::Result<T, DeepSeekError>;

impl DeepSeekError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            DeepSeekError::HttpError(_) 
            | DeepSeekError::RateLimitExceeded
            | DeepSeekError::TimeoutError(_)
        )
    }
    
    /// Get the HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            DeepSeekError::ApiError { status, .. } => Some(*status),
            _ => None,
        }
    }
    
    /// Check if this is an authentication error
    pub fn is_auth_error(&self) -> bool {
        matches!(self, DeepSeekError::AuthenticationError(_))
            || matches!(self, DeepSeekError::ApiError { status: 401 | 403, .. })
    }
    
    /// Check if this is a rate limit error
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, DeepSeekError::RateLimitExceeded)
            || matches!(self, DeepSeekError::ApiError { status: 429, .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_is_retryable() {
        let timeout_err = DeepSeekError::TimeoutError(30);
        assert!(timeout_err.is_retryable());
        
        let config_err = DeepSeekError::ConfigError("test".to_string());
        assert!(!config_err.is_retryable());
        
        let rate_limit_err = DeepSeekError::RateLimitExceeded;
        assert!(rate_limit_err.is_retryable());
    }
    
    #[test]
    fn test_error_status_code() {
        let api_err = DeepSeekError::ApiError {
            status: 404,
            message: "Not found".to_string(),
        };
        assert_eq!(api_err.status_code(), Some(404));
        
        let http_err = DeepSeekError::ConfigError("test".to_string());
        assert_eq!(http_err.status_code(), None);
    }
    
    #[test]
    fn test_is_auth_error() {
        let auth_err = DeepSeekError::AuthenticationError("Invalid API key".to_string());
        assert!(auth_err.is_auth_error());
        
        let api_401 = DeepSeekError::ApiError {
            status: 401,
            message: "Unauthorized".to_string(),
        };
        assert!(api_401.is_auth_error());
        
        let api_403 = DeepSeekError::ApiError {
            status: 403,
            message: "Forbidden".to_string(),
        };
        assert!(api_403.is_auth_error());
        
        let other_err = DeepSeekError::ConfigError("test".to_string());
        assert!(!other_err.is_auth_error());
    }
    
    #[test]
    fn test_is_rate_limit() {
        let rate_err = DeepSeekError::RateLimitExceeded;
        assert!(rate_err.is_rate_limit());
        
        let api_429 = DeepSeekError::ApiError {
            status: 429,
            message: "Too many requests".to_string(),
        };
        assert!(api_429.is_rate_limit());
        
        let other_err = DeepSeekError::ConfigError("test".to_string());
        assert!(!other_err.is_rate_limit());
    }
}