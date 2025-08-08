//! Request models for DeepSeek API

use crate::error::{DeepSeekError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Available DeepSeek models
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    /// DeepSeek Chat model for general conversations
    #[serde(rename = "deepseek-chat")]
    Chat,
    
    /// DeepSeek Reasoner model for complex reasoning tasks
    #[serde(rename = "deepseek-reasoner")]
    Reasoner,
    
    /// DeepSeek Coder model for programming tasks
    #[serde(rename = "deepseek-coder")]
    Coder,
}

impl Model {
    /// Get the model's string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::Chat => "deepseek-chat",
            Model::Reasoner => "deepseek-reasoner",
            Model::Coder => "deepseek-coder",
        }
    }
    
    /// Check if this model supports reasoning
    pub fn supports_reasoning(&self) -> bool {
        matches!(self, Model::Reasoner)
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::Chat
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Message role in conversation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System message to set context
    System,
    /// User message
    User,
    /// Assistant response
    Assistant,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
        }
    }
}

/// A message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    /// The role of the message sender
    pub role: Role,
    
    /// The content of the message
    pub content: String,
}

impl Message {
    /// Create a new message with a specific role
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
    
    /// Create a system message
    /// 
    /// # Example
    /// ```
    /// use deepseek_rust::Message;
    /// 
    /// let msg = Message::system("You are a helpful assistant");
    /// ```
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }
    
    /// Create a user message
    /// 
    /// # Example
    /// ```
    /// use deepseek_rust::Message;
    /// 
    /// let msg = Message::user("Hello, how are you?");
    /// ```
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }
    
    /// Create an assistant message
    /// 
    /// # Example
    /// ```
    /// use deepseek_rust::Message;
    /// 
    /// let msg = Message::assistant("I'm doing well, thank you!");
    /// ```
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }
    
    /// Get the length of the message content
    pub fn len(&self) -> usize {
        self.content.len()
    }
    
    /// Check if the message is empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

/// Temperature for response randomness (0.0 - 2.0)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature(f32);

impl Temperature {
    /// Create a new temperature value
    /// 
    /// # Arguments
    /// * `value` - Temperature value between 0.0 and 2.0
    /// 
    /// # Errors
    /// Returns an error if the value is outside the valid range
    pub fn new(value: f32) -> Result<Self> {
        if !(0.0..=2.0).contains(&value) {
            return Err(DeepSeekError::InvalidParameter(
                format!("Temperature must be between 0.0 and 2.0, got {}", value)
            ));
        }
        Ok(Self(value))
    }
    
    /// Create a temperature value without validation (unsafe)
    pub fn new_unchecked(value: f32) -> Self {
        Self(value)
    }
    
    /// Get the temperature value
    pub fn value(&self) -> f32 {
        self.0
    }
    
    /// Very low randomness (0.1)
    pub fn very_low() -> Self {
        Self(0.1)
    }
    
    /// Low randomness (0.3)
    pub fn low() -> Self {
        Self(0.3)
    }
    
    /// Medium randomness (0.7)
    pub fn medium() -> Self {
        Self(0.7)
    }
    
    /// High randomness (1.0)
    pub fn high() -> Self {
        Self(1.0)
    }
    
    /// Very high randomness (1.5)
    pub fn very_high() -> Self {
        Self(1.5)
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Self::medium()
    }
}

impl Serialize for Temperature {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Temperature {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        Temperature::new(value).map_err(serde::de::Error::custom)
    }
}

/// Chat completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// The model to use
    pub model: Model,
    
    /// The messages in the conversation
    pub messages: Vec<Message>,
    
    /// Temperature for randomness (0.0-2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,
    
    /// Maximum tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    
    /// Top-p sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    
    /// Frequency penalty (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    
    /// Presence penalty (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    
    /// Number of completions to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    
    /// User identifier for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ChatCompletionRequest {
    /// Create a new request with messages
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            model: Model::default(),
            messages,
            temperature: None,
            max_tokens: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stop: None,
            stream: None,
            n: None,
            user: None,
        }
    }
    
    /// Create a request with a single user message
    pub fn from_user_message(content: impl Into<String>) -> Self {
        Self::new(vec![Message::user(content)])
    }
    
    /// Set the model
    pub fn with_model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }
    
    /// Set temperature
    pub fn with_temperature(mut self, temp: Temperature) -> Self {
        self.temperature = Some(temp);
        self
    }
    
    /// Set max tokens
    pub fn with_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }
    
    /// Set top-p sampling
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }
    
    /// Set frequency penalty
    pub fn with_frequency_penalty(mut self, penalty: f32) -> Self {
        self.frequency_penalty = Some(penalty);
        self
    }
    
    /// Set presence penalty
    pub fn with_presence_penalty(mut self, penalty: f32) -> Self {
        self.presence_penalty = Some(penalty);
        self
    }
    
    /// Set stop sequences
    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }
    
    /// Enable streaming
    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
    
    /// Set number of completions
    pub fn with_n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }
    
    /// Set user identifier
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
    
    /// Validate the request
    pub fn validate(&self) -> Result<()> {
        // Check messages
        if self.messages.is_empty() {
            return Err(DeepSeekError::InvalidParameter(
                "At least one message is required".to_string()
            ));
        }
        
        // Check for empty messages
        for (i, msg) in self.messages.iter().enumerate() {
            if msg.is_empty() {
                return Err(DeepSeekError::InvalidParameter(
                    format!("Message at index {} is empty", i)
                ));
            }
        }
        
        // Validate top_p
        if let Some(top_p) = self.top_p {
            if !(0.0..=1.0).contains(&top_p) {
                return Err(DeepSeekError::InvalidParameter(
                    format!("top_p must be between 0.0 and 1.0, got {}", top_p)
                ));
            }
        }
        
        // Validate frequency_penalty
        if let Some(penalty) = self.frequency_penalty {
            if !(-2.0..=2.0).contains(&penalty) {
                return Err(DeepSeekError::InvalidParameter(
                    format!("frequency_penalty must be between -2.0 and 2.0, got {}", penalty)
                ));
            }
        }
        
        // Validate presence_penalty
        if let Some(penalty) = self.presence_penalty {
            if !(-2.0..=2.0).contains(&penalty) {
                return Err(DeepSeekError::InvalidParameter(
                    format!("presence_penalty must be between -2.0 and 2.0, got {}", penalty)
                ));
            }
        }
        
        // Validate n
        if let Some(n) = self.n {
            if n == 0 || n > 10 {
                return Err(DeepSeekError::InvalidParameter(
                    format!("n must be between 1 and 10, got {}", n)
                ));
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_model_string_conversion() {
        assert_eq!(Model::Chat.as_str(), "deepseek-chat");
        assert_eq!(Model::Reasoner.as_str(), "deepseek-reasoner");
        assert_eq!(Model::Coder.as_str(), "deepseek-coder");
    }
    
    #[test]
    fn test_model_supports_reasoning() {
        assert!(!Model::Chat.supports_reasoning());
        assert!(Model::Reasoner.supports_reasoning());
        assert!(!Model::Coder.supports_reasoning());
    }
    
    #[test]
    fn test_message_creation() {
        let system_msg = Message::system("System prompt");
        assert_eq!(system_msg.role, Role::System);
        assert_eq!(system_msg.content, "System prompt");
        
        let user_msg = Message::user("User input");
        assert_eq!(user_msg.role, Role::User);
        assert_eq!(user_msg.content, "User input");
        
        let assistant_msg = Message::assistant("Assistant response");
        assert_eq!(assistant_msg.role, Role::Assistant);
        assert_eq!(assistant_msg.content, "Assistant response");
    }
    
    #[test]
    fn test_temperature_validation() {
        assert!(Temperature::new(0.5).is_ok());
        assert!(Temperature::new(0.0).is_ok());
        assert!(Temperature::new(2.0).is_ok());
        assert!(Temperature::new(-0.1).is_err());
        assert!(Temperature::new(2.1).is_err());
    }
    
    #[test]
    fn test_temperature_presets() {
        assert_eq!(Temperature::very_low().value(), 0.1);
        assert_eq!(Temperature::low().value(), 0.3);
        assert_eq!(Temperature::medium().value(), 0.7);
        assert_eq!(Temperature::high().value(), 1.0);
        assert_eq!(Temperature::very_high().value(), 1.5);
    }
    
    #[test]
    fn test_request_validation() {
        // Empty messages
        let empty_request = ChatCompletionRequest::new(vec![]);
        assert!(empty_request.validate().is_err());
        
        // Valid request
        let valid_request = ChatCompletionRequest::new(vec![
            Message::user("Hello")
        ]);
        assert!(valid_request.validate().is_ok());
        
        // Invalid top_p
        let invalid_top_p = ChatCompletionRequest::new(vec![Message::user("Hi")])
            .with_top_p(1.5);
        assert!(invalid_top_p.validate().is_err());
        
        // Invalid frequency_penalty
        let invalid_freq = ChatCompletionRequest::new(vec![Message::user("Hi")])
            .with_frequency_penalty(3.0);
        assert!(invalid_freq.validate().is_err());
    }
}