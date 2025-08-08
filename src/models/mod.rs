//! Data models for DeepSeek API requests and responses

pub mod request;
pub mod response;

// Re-export commonly used types
pub use request::{
    ChatCompletionRequest, Message, Model, Role, Temperature,
};
pub use response::{
    ApiErrorDetail, ApiErrorResponse, ChatCompletionResponse, Choice, ResponseMessage, Usage,
};