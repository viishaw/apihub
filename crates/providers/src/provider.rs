//! Provider trait 和通用类型

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

/// Provider 错误类型
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Rate limited")]
    RateLimited,
    
    #[error("Invalid response")]
    InvalidResponse,
    
    #[error("Stream error: {0}")]
    Stream(String),
}

/// 聊天响应
#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub model: String,
    pub content: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub finish_reason: String,
}

/// 流式响应
#[derive(Debug, Clone)]
pub struct StreamResponse {
    pub delta: String,
    pub finish_reason: Option<String>,
}

/// Provider trait
#[async_trait]
pub trait Provider: Send + Sync {
    /// 发送聊天请求
    async fn chat(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: f32,
        max_tokens: Option<u32>,
    ) -> Result<ChatResponse, ProviderError>;
    
    /// 发送流式聊天请求
    async fn chat_stream(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: f32,
        max_tokens: Option<u32>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamResponse, ProviderError>> + Send>>, ProviderError>;
    
    /// 列出支持的模型
    fn supported_models(&self) -> Vec<&'static str>;
    
    /// Provider 名称
    fn name(&self) -> &'static str;
}

/// 消息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
    
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }
    
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}
