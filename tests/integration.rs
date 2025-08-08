//! Integration tests for DeepSeek Rust client

use deepseek_rust::{
    ChatCompletionRequest, DeepSeekClient, DeepSeekConfig, DeepSeekError, Message, Model,
    Result, Temperature,
};
use mockito::{mock, server_url, Matcher};
use serde_json::json;

/// Helper function to create a test client with mock server
fn create_test_client() -> DeepSeekClient {
    let config = DeepSeekConfig::new("test-api-key")
        .with_base_url(server_url())
        .with_max_retries(1);
    
    DeepSeekClient::new(config).expect("Failed to create test client")
}

/// Helper function to create a mock successful response
fn mock_success_response() -> serde_json::Value {
    json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1677652288,
        "model": "deepseek-chat",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Hello! How can I help you today?"
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 8,
            "total_tokens": 18
        }
    })
}

/// Helper function to create a mock error response
fn mock_error_response(status: u16, message: &str) -> serde_json::Value {
    json!({
        "error": {
            "message": message,
            "type": "invalid_request_error",
            "code": "invalid_api_key"
        }
    })
}

#[tokio::test]
async fn test_simple_chat_completion() {
    let _mock = mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_success_response().to_string())
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .expect(1)
        .create();

    let client = create_test_client();
    let response = client
        .chat()
        .add_user_message("Hello")
        .send()
        .await
        .expect("Request should succeed");

    assert_eq!(response.choices.len(), 1);
    assert_eq!(
        response.choices[0].message.content.as_ref().unwrap(),
        "Hello! How can I help you today?"
    );
    assert_eq!(response.usage.as_ref().unwrap().total_tokens, 18);
}

#[tokio::test]
async fn test_chat_with_system_message() {
    let _mock = mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_success_response().to_string())
        .match_body(Matcher::PartialJson(json!({
            "messages": [
                {"role": "system", "content": "You are a helpful assistant"},
                {"role": "user", "content": "Hello"}
            ]
        })))
        .create();

    let client = create_test_client();
    let response = client
        .chat()
        .add_system_message("You are a helpful assistant")
        .add_user_message("Hello")
        .send()
        .await
        .expect("Request should succeed");

    assert!(response.choices[0].message.content.is_some());
}

#[tokio::test]
async fn test_chat_with_parameters() {
    let _mock = mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_success_response().to_string())
        .match_body(Matcher::PartialJson(json!({
            "model": "deepseek-coder",
            "temperature": 0.5,
            "max_tokens": 100
        })))
        .create();

    let client = create_test_client();
    let response = client
        .chat()
        .add_user_message("Write code")
        .with_model(Model::Coder)
        .with_temperature(0.5)
        .expect("Temperature should be valid")
        .with_max_tokens(100)
        .send()
        .await
        .expect("Request should succeed");

    assert!(response.choices[0].message.content.is_some());
}

#[tokio::test]
async fn test_reasoning_model_response() {
    let reasoning_response = json!({
        "id": "chatcmpl-reasoning",
        "object": "chat.completion",
        "created": 1677652288,
        "model": "deepseek-reasoner",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "The answer is 42",
                "reasoning_content": "Let me think step by step..."
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 8,
            "total_tokens": 18,
            "reasoning_tokens": 5
        }
    });

    let _mock = mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(reasoning_response.to_string())
        .match_body(Matcher::PartialJson(json!({
            "model": "deepseek-reasoner"
        })))
        .create();

    let client = create_test_client();
    let response = client
        .chat()
        .add_user_message("What is 6 * 7?")
        .with_model(Model::Reasoner)
        .send()
        .await
        .expect("Request should succeed");

    assert_eq!(
        response.choices[0].message.content.as_ref().unwrap(),
        "The answer is 42"
    );
    assert_eq!(
        response.choices[0].message.reasoning_content.as_ref().unwrap(),
        "Let me think step by step..."
    );
    assert_eq!(response.usage.as_ref().unwrap().reasoning_tokens, Some(5));
}

#[tokio::test]
async fn test_authentication_error() {
    let _mock = mock("POST", "/chat/completions")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(mock_error_response(401, "Invalid API key").to_string())
        .create();

    let client = create_test_client();
    let result = client
        .chat()
        .add_user_message("Hello")
        .send()
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        DeepSeekError::InvalidParameter(msg) => {
            assert!(msg.contains("At least one message is required"));
        }
        _ => panic!("Expected InvalidParameter error"),
    }
}

#[tokio::test]
async fn test_invalid_temperature() {
    let client = create_test_client();
    let result = client
        .chat()
        .add_user_message("Hello")
        .with_temperature(3.0); // Invalid: > 2.0

    assert!(result.is_err());
    match result.unwrap_err() {
        DeepSeekError::InvalidParameter(msg) => {
            assert!(msg.contains("Temperature must be between 0.0 and 2.0"));
        }
        _ => panic!("Expected InvalidParameter error"),
    }
}

#[tokio::test]
async fn test_multi_turn_conversation() {
    let _mock = mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_success_response().to_string())
        .match_body(Matcher::PartialJson(json!({
            "messages": [
                {"role": "user", "content": "My name is Alice"},
                {"role": "assistant", "content": "Nice to meet you, Alice!"},
                {"role": "user", "content": "What's my name?"}
            ]
        })))
        .create();

    let client = create_test_client();
    let response = client
        .chat()
        .add_user_message("My name is Alice")
        .add_assistant_message("Nice to meet you, Alice!")
        .add_user_message("What's my name?")
        .send()
        .await
        .expect("Request should succeed");

    assert!(response.choices[0].message.content.is_some());
}

#[tokio::test]
async fn test_request_with_all_parameters() {
    let _mock = mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_success_response().to_string())
        .match_body(Matcher::PartialJson(json!({
            "model": "deepseek-chat",
            "temperature": 0.7,
            "max_tokens": 500,
            "top_p": 0.9,
            "frequency_penalty": 0.5,
            "presence_penalty": 0.3,
            "stop": ["END"],
            "n": 2,
            "user": "test-user"
        })))
        .create();

    let request = ChatCompletionRequest::new(vec![Message::user("Hello")])
        .with_model(Model::Chat)
        .with_temperature(Temperature::medium())
        .with_max_tokens(500)
        .with_top_p(0.9)
        .with_frequency_penalty(0.5)
        .with_presence_penalty(0.3)
        .with_stop(vec!["END".to_string()])
        .with_n(2)
        .with_user("test-user");

    let client = create_test_client();
    let response = client
        .chat_completion(request)
        .await
        .expect("Request should succeed");

    assert!(response.choices[0].message.content.is_some());
}

#[tokio::test]
async fn test_retry_on_transient_error() {
    // First request fails with 500, second succeeds
    let _mock_fail = mock("POST", "/chat/completions")
        .with_status(500)
        .with_body("Internal Server Error")
        .expect(1)
        .create();

    let _mock_success = mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_success_response().to_string())
        .expect(1)
        .create();

    let client = create_test_client();
    let response = client
        .chat()
        .add_user_message("Hello")
        .send()
        .await
        .expect("Request should succeed after retry");

    assert!(response.choices[0].message.content.is_some());
}

#[tokio::test]
async fn test_connection_test() {
    let _mock = mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_success_response().to_string())
        .create();

    let client = create_test_client();
    let result = client.test_connection().await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_builder_pattern_chaining() {
    let client = create_test_client();
    
    // Test that all builder methods can be chained
    let builder = client
        .chat()
        .add_system_message("System")
        .add_user_message("User")
        .add_assistant_message("Assistant")
        .with_model(Model::Chat)
        .with_max_tokens(100);

    assert_eq!(builder.messages.len(), 3);
    assert_eq!(builder.model, Model::Chat);
    assert_eq!(builder.max_tokens, Some(100));
}

#[tokio::test]
async fn test_response_helpers() {
    let response_json = json!({
        "id": "test",
        "object": "chat.completion",
        "created": 1234567890,
        "model": "deepseek-chat",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Test content",
                "reasoning_content": "Test reasoning"
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 5,
            "total_tokens": 15
        }
    });

    let response: deepseek_rust::ChatCompletionResponse = 
        serde_json::from_value(response_json).unwrap();

    assert_eq!(response.get_content(), Some("Test content"));
    assert_eq!(response.get_reasoning(), Some("Test reasoning"));
    assert!(response.is_finished());
    assert_eq!(response.total_tokens(), Some(15));
}

// Test with environment variables
#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn test_from_env() {
    // This test requires DEEPSEEK_API_KEY to be set
    let result = DeepSeekClient::from_env();
    
    if std::env::var("DEEPSEEK_API_KEY").is_ok() {
        assert!(result.is_ok());
        let client = result.unwrap();
        
        // Test connection (will make real API call)
        let connection_result = client.test_connection().await;
        assert!(connection_result.is_ok());
    } else {
        assert!(result.is_err());
    }
}

// Performance test
#[tokio::test]
async fn test_concurrent_requests() {
    use futures::future::join_all;
    
    let _mock = mock("POST", "/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_success_response().to_string())
        .expect_at_least(3)
        .create();

    let client = create_test_client();
    
    let futures = vec![
        client.chat().add_user_message("Hello 1").send(),
        client.chat().add_user_message("Hello 2").send(),
        client.chat().add_user_message("Hello 3").send(),
    ];
    
    let results = join_all(futures).await;
    
    for result in results {
        assert!(result.is_ok());
    }
}());
    let error = result.unwrap_err();
    assert!(error.is_auth_error());
    
    match error {
        DeepSeekError::ApiError { status, message } => {
            assert_eq!(status, 401);
            assert!(message.contains("Invalid API key"));
        }
        _ => panic!("Expected ApiError"),
    }
}

#[tokio::test]
async fn test_rate_limit_error() {
    let _mock = mock("POST", "/chat/completions")
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_body(mock_error_response(429, "Rate limit exceeded").to_string())
        .create();

    let client = create_test_client();
    let result = client
        .chat()
        .add_user_message("Hello")
        .send()
        .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_rate_limit());
}

#[tokio::test]
async fn test_empty_messages_validation() {
    let client = create_test_client();
    let result = client.chat().send().await;

    assert!(result.is_err