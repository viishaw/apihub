//! Anthropic Provider

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use tokio_stream::wrappers::ReceiverStream;

use crate::provider::{Provider, ProviderError, ChatResponse, StreamResponse, Message};

/// Anthropic Provider
pub struct AnthropicProvider {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.unwrap_or_else(|| "https://api.anthropic.com/v1".to_string()),
            api_key,
        }
    }
    
    fn convert_messages(&self, messages: Vec<Message>) -> (String, Vec<serde_json::Value>) {
        let mut system = String::new();
        let mut converted = Vec::new();
        
        for msg in messages {
            if msg.role == "system" {
                system = msg.content;
            } else {
                converted.push(serde_json::json!({
                    "role": msg.role,
                    "content": msg.content,
                }));
            }
        }
        
        (system, converted)
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    async fn chat(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: f32,
        max_tokens: Option<u32>,
    ) -> Result<ChatResponse, ProviderError> {
        let (system, messages) = self.convert_messages(messages);
        
        let mut body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": max_tokens.unwrap_or(4096),
        });
        
        if !system.is_empty() {
            body["system"] = serde_json::json!(system);
        }
        
        if temperature != 1.0 {
            body["temperature"] = serde_json::json!(temperature);
        }
        
        let response = self.client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(ProviderError::Api(error));
        }
        
        let json: serde_json::Value = response.json().await?;
        
        let content = json["content"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|block| block["text"].as_str())
            .unwrap_or("")
            .to_string();
        
        let input_tokens = json["usage"]["input_tokens"]
            .as_u64()
            .unwrap_or(0) as u32;
        
        let output_tokens = json["usage"]["output_tokens"]
            .as_u64()
            .unwrap_or(0) as u32;
        
        let finish_reason = json["stop_reason"]
            .as_str()
            .unwrap_or("end_turn")
            .to_string();
        
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
        let (system, messages) = self.convert_messages(messages);
        
        let mut body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": max_tokens.unwrap_or(4096),
            "stream": true,
        });
        
        if !system.is_empty() {
            body["system"] = serde_json::json!(system);
        }
        
        if temperature != 1.0 {
            body["temperature"] = serde_json::json!(temperature);
        }
        
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        
        let client = self.client.clone();
        let url = format!("{}/messages", self.base_url);
        let api_key = self.api_key.clone();
        
        tokio::spawn(async move {
            let response = match client
                .post(&url)
                .header("x-api-key", &api_key)
                .header("anthropic-version", "2023-06-01")
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
                                
                                let json: serde_json::Value = match serde_json::from_str(data) {
                                    Ok(j) => j,
                                    Err(_) => continue,
                                };
                                
                                let event_type = json["type"].as_str().unwrap_or("");
                                
                                match event_type {
                                    "content_block_delta" => {
                                        let delta = json["delta"]["text"]
                                            .as_str()
                                            .unwrap_or("")
                                            .to_string();
                                        
                                        if !delta.is_empty() {
                                            let _ = tx.send(Ok(StreamResponse {
                                                delta,
                                                finish_reason: None,
                                            })).await;
                                        }
                                    }
                                    "message_stop" => {
                                        let _ = tx.send(Ok(StreamResponse {
                                            delta: String::new(),
                                            finish_reason: Some("end_turn".to_string()),
                                        })).await;
                                        return;
                                    }
                                    _ => {}
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
            "claude-3-opus-20240229",
            "claude-3-sonnet-20240229",
            "claude-3-haiku-20240307",
            "claude-2.1",
            "claude-2.0",
        ]
    }
    
    fn name(&self) -> &'static str {
        "anthropic"
    }
}
