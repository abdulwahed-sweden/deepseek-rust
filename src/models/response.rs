//! Response models for DeepSeek API

use serde::{Deserialize, Serialize};

/// Chat completion response from the API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionResponse {
    /// Unique identifier for the completion
    pub id: String,
    
    /// Object type (usually "chat.completion")
    pub object: String,
    
    /// Unix timestamp of when the completion was created
    pub created: u64,
    
    /// The model used for the completion
    pub model: String,
    
    /// List of completion choices
    pub choices: Vec<Choice>,
    
    /// Token usage information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    
    /// System fingerprint for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
}

impl ChatCompletionResponse {
    /// Get the first choice's content if available
    pub fn get_content(&self) -> Option<&str> {
        self.choices
            .first()
            .and_then(|choice| choice.message.content.as_deref())
    }
    
    /// Get the first choice's reasoning content if available
    pub fn get_reasoning(&self) -> Option<&str> {
        self.choices
            .first()
            .and_then(|choice| choice.message.reasoning_content.as_deref())
    }
    
    /// Check if the response was finished
    pub fn is_finished(&self) -> bool {
        self.choices
            .first()
            .map(|choice| choice.finish_reason.as_deref() == Some("stop"))
            .unwrap_or(false)
    }
    
    /// Get total tokens used
    pub fn total_tokens(&self) -> Option<u32> {
        self.usage.as_ref().map(|u| u.total_tokens)
    }
}

/// A choice in the completion response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Choice {
    /// The index of this choice
    pub index: u32,
    
    /// The message content
    pub message: ResponseMessage,
    
    /// The reason the completion stopped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    
    /// Log probabilities (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<serde_json::Value>,
}

/// Response message from the assistant
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseMessage {
    /// The role of the message (usually "assistant")
    pub role: String,
    
    /// The main content of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    
    /// Reasoning content (for reasoning models)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    
    /// Function call information (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
    
    /// Tool calls (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl ResponseMessage {
    /// Check if the message has content
    pub fn has_content(&self) -> bool {
        self.content.is_some() && !self.content.as_ref().unwrap().is_empty()
    }
    
    /// Check if the message has reasoning content
    pub fn has_reasoning(&self) -> bool {
        self.reasoning_content.is_some() && !self.reasoning_content.as_ref().unwrap().is_empty()
    }
    
    /// Get the total length of all content
    pub fn total_length(&self) -> usize {
        let content_len = self.content.as_ref().map(|s| s.len()).unwrap_or(0);
        let reasoning_len = self.reasoning_content.as_ref().map(|s| s.len()).unwrap_or(0);
        content_len + reasoning_len
    }
}

/// Function call information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionCall {
    /// The name of the function to call
    pub name: String,
    
    /// The arguments to pass to the function (as JSON string)
    pub arguments: String,
}

/// Tool call information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolCall {
    /// Unique identifier for the tool call
    pub id: String,
    
    /// The type of tool (usually "function")
    pub r#type: String,
    
    /// The function call details
    pub function: FunctionCall,
}

/// Token usage information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Usage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    
    /// Total number of tokens used
    pub total_tokens: u32,
    
    /// Number of reasoning tokens (for reasoning models)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
    
    /// Cached tokens (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_hit_tokens: Option<u32>,
    
    /// Cache miss tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_miss_tokens: Option<u32>,
}

impl Usage {
    /// Calculate the total cost (simplified example)
    /// Note: You should adjust these rates based on actual pricing
    pub fn estimate_cost(&self) -> f64 {
        const PROMPT_RATE: f64 = 0.0001;  // per token
        const COMPLETION_RATE: f64 = 0.0002;  // per token
        
        let prompt_cost = self.prompt_tokens as f64 * PROMPT_RATE;
        let completion_cost = self.completion_tokens as f64 * COMPLETION_RATE;
        
        prompt_cost + completion_cost
    }
}

/// API Error response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiErrorResponse {
    /// The error details
    pub error: ApiErrorDetail,
}

/// API Error details
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiErrorDetail {
    /// Error message
    pub message: String,
    
    /// Error type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    
    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    
    /// Additional error parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

/// Streaming response chunk
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamChunk {
    /// Unique identifier for the chunk
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// Creation timestamp
    pub created: u64,
    
    /// Model used
    pub model: String,
    
    /// Choices in this chunk
    pub choices: Vec<StreamChoice>,
}

/// A choice in a streaming response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamChoice {
    /// Index of the choice
    pub index: u32,
    
    /// Delta content
    pub delta: DeltaContent,
    
    /// Finish reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// Delta content in streaming responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeltaContent {
    /// Role (only in first chunk)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    
    /// Content delta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    
    /// Reasoning content delta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_response_helpers() {
        let response = ChatCompletionResponse {
            id: "test-id".to_string(),
            object: "chat.completion".to_string(),
            created: 1234567890,
            model: "deepseek-chat".to_string(),
            choices: vec![
                Choice {
                    index: 0,
                    message: ResponseMessage {
                        role: "assistant".to_string(),
                        content: Some("Hello!".to_string()),
                        reasoning_content: Some("Reasoning here".to_string()),
                        function_call: None,
                        tool_calls: None,
                    },
                    finish_reason: Some("stop".to_string()),
                    logprobs: None,
                }
            ],
            usage: Some(Usage {
                prompt_tokens: 10,
                completion_tokens: 5,
                total_tokens: 15,
                reasoning_tokens: Some(3),
                prompt_cache_hit_tokens: None,
                prompt_cache_miss_tokens: None,
            }),
            system_fingerprint: None,
        };
        
        assert_eq!(response.get_content(), Some("Hello!"));
        assert_eq!(response.get_reasoning(), Some("Reasoning here"));
        assert!(response.is_finished());
        assert_eq!(response.total_tokens(), Some(15));
    }
    
    #[test]
    fn test_message_helpers() {
        let message = ResponseMessage {
            role: "assistant".to_string(),
            content: Some("Hello world!".to_string()),
            reasoning_content: Some("This is reasoning".to_string()),
            function_call: None,
            tool_calls: None,
        };
        
        assert!(message.has_content());
        assert!(message.has_reasoning());
        assert_eq!(message.total_length(), 28); // "Hello world!" (12) + "This is reasoning" (16)
    }
    
    #[test]
    fn test_usage_cost_estimation() {
        let usage = Usage {
            prompt_tokens: 100,
            completion_tokens: 50,
            total_tokens: 150,
            reasoning_tokens: None,
            prompt_cache_hit_tokens: None,
            prompt_cache_miss_tokens: None,
        };
        
        let cost = usage.estimate_cost();
        assert!((cost - 0.02).abs() < 0.0001); // 100 * 0.0001 + 50 * 0.0002 = 0.02
    }
}