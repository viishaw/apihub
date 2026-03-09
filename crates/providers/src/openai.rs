//! OpenAI Provider

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use tokio_stream::wrappers::ReceiverStream;

use crate::provider::{Provider, ProviderError, ChatResponse, StreamResponse, Message};

/// OpenAI Provider
pub struct OpenAIProvider {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            api_key,
        }
    }
    
    fn build_request_body(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: f32,
        max_tokens: Option<u32>,
        stream: bool,
    ) -> serde_json::Value {
        let mut body = serde_json::json!({
            "model": model,
            "messages": messages,
            "temperature": temperature,
            "stream": stream,
        });
        
        if let Some(max) = max_tokens {
            body["max_tokens"] = serde_json::json!(max);
        }
        
        body
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn chat(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: f32,
        max_tokens: Option<u32>,
    ) -> Result<ChatResponse, ProviderError> {
        let body = self.build_request_body(model, messages, temperature, max_tokens, false);
        
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(ProviderError::Api(error));
        }
        
        let json: serde_json::Value = response.json().await?;
        
        let choice = json["choices"]
            .as_array()
            .and_then(|arr| arr.first())
            .ok_or(ProviderError::InvalidResponse)?;
        
        let content = choice["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        
        let finish_reason = choice["finish_reason"]
            .as_str()
            .unwrap_or("stop")
            .to_string();
        
        let usage = &json["usage"];
        let input_tokens = usage["prompt_tokens"].as_u64().unwrap_or(0) as u32;
        let output_tokens = usage["completion_tokens"].as_u64().unwrap_or(0) as u32;
        
        Ok(ChatResponse {
            id: json["id"].as_str().unwrap_or("").to_string(),
            model: json["model"].as_str().unwrap_or(model).to_string(),
            content,
            input_tokens,
            output_tokens,
            finish_reason,
        })
    }
    
    async fn chat_stream(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: f32,
        max_tokens: Option<u32>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamResponse, ProviderError>> + Send>>, ProviderError> {
        let body = self.build_request_body(model, messages, temperature, max_tokens, true);
        
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        
        let client = self.client.clone();
        let url = format!("{}/chat/completions", self.base_url);
        let api_key = self.api_key.clone();
        
        tokio::spawn(async move {
            let response = match client
                .post(&url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    let _ = tx.send(Err(ProviderError::Http(e))).await;
                    return;
                }
            };
            
            let mut stream = response.bytes_stream();
            
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" {
                                    return;
                                }
                                
                                match serde_json::from_str::<serde_json::Value>(data) {
                                    Ok(json) => {
                                        let delta = json["choices"]
                                            .as_array()
                                            .and_then(|arr| arr.first())
                                            .and_then(|choice| choice["delta"]["content"].as_str())
                                            .unwrap_or("")
                                            .to_string();
                                        
                                        let finish_reason = json["choices"]
                                            .as_array()
                                            .and_then(|arr| arr.first())
                                            .and_then(|choice| choice["finish_reason"].as_str())
                                            .map(|s| s.to_string());
                                        
                                        if !delta.is_empty() || finish_reason.is_some() {
                                            let _ = tx.send(Ok(StreamResponse {
                                                delta,
                                                finish_reason,
                                            })).await;
                                        }
                                    }
                                    Err(_) => continue,
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(ProviderError::Stream(e.to_string()))).await;
                        return;
                    }
                }
            }
        });
        
        Ok(Box::pin(ReceiverStream::new(rx)))
    }
    
    fn supported_models(&self) -> Vec<&'static str> {
        vec![
            "gpt-4-turbo",
            "gpt-4-turbo-preview",
            "gpt-4",
            "gpt-4-32k",
            "gpt-3.5-turbo",
            "gpt-3.5-turbo-16k",
        ]
    }
    
    fn name(&self) -> &'static str {
        "openai"
    }
}
