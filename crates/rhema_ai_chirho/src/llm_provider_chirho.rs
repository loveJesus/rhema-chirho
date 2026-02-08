// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! LLM provider abstraction — trait + implementations for different backends.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error_chirho::AiResultChirho;

/// A message in a chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageChirho {
    pub role_chirho: String,
    pub content_chirho: String,
}

/// Response from an LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponseChirho {
    pub content_chirho: String,
    pub model_chirho: String,
    pub usage_chirho: Option<LlmUsageChirho>,
}

/// Token usage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmUsageChirho {
    pub prompt_tokens_chirho: u32,
    pub completion_tokens_chirho: u32,
    pub total_tokens_chirho: u32,
}

/// Trait for LLM provider implementations.
#[async_trait]
pub trait LlmProviderChirho: Send + Sync {
    /// Send a chat completion request.
    async fn chat_completion_chirho(
        &self,
        messages_chirho: &[ChatMessageChirho],
        max_tokens_chirho: usize,
        temperature_chirho: f32,
    ) -> AiResultChirho<LlmResponseChirho>;

    /// Get the provider name.
    fn provider_name_chirho(&self) -> &str;

    /// Get the model name.
    fn model_name_chirho(&self) -> &str;
}

/// Mock LLM provider for testing — returns canned responses.
pub struct MockLlmProviderChirho {
    model_chirho: String,
    response_chirho: String,
}

impl MockLlmProviderChirho {
    pub fn new_chirho() -> Self {
        Self {
            model_chirho: "mock-llm".to_string(),
            response_chirho: String::new(),
        }
    }

    pub fn with_response_chirho(mut self, response_chirho: &str) -> Self {
        self.response_chirho = response_chirho.to_string();
        self
    }
}

#[async_trait]
impl LlmProviderChirho for MockLlmProviderChirho {
    async fn chat_completion_chirho(
        &self,
        _messages_chirho: &[ChatMessageChirho],
        _max_tokens_chirho: usize,
        _temperature_chirho: f32,
    ) -> AiResultChirho<LlmResponseChirho> {
        Ok(LlmResponseChirho {
            content_chirho: self.response_chirho.clone(),
            model_chirho: self.model_chirho.clone(),
            usage_chirho: Some(LlmUsageChirho {
                prompt_tokens_chirho: 10,
                completion_tokens_chirho: 20,
                total_tokens_chirho: 30,
            }),
        })
    }

    fn provider_name_chirho(&self) -> &str {
        "mock"
    }

    fn model_name_chirho(&self) -> &str {
        &self.model_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider_chirho() {
        let provider_chirho =
            MockLlmProviderChirho::new_chirho().with_response_chirho("Test response");

        let messages_chirho = vec![ChatMessageChirho {
            role_chirho: "user".to_string(),
            content_chirho: "Hello".to_string(),
        }];

        let response_chirho = provider_chirho
            .chat_completion_chirho(&messages_chirho, 100, 0.3)
            .await
            .unwrap();

        assert_eq!(response_chirho.content_chirho, "Test response");
        assert_eq!(response_chirho.model_chirho, "mock-llm");
        assert!(response_chirho.usage_chirho.is_some());
    }

    #[test]
    fn test_provider_name_chirho() {
        let provider_chirho = MockLlmProviderChirho::new_chirho();
        assert_eq!(provider_chirho.provider_name_chirho(), "mock");
        assert_eq!(provider_chirho.model_name_chirho(), "mock-llm");
    }
}
