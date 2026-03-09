//! Providers crate - API 提供商适配器

pub mod openai;
pub mod anthropic;
pub mod provider;

pub use provider::{Provider, ProviderError, ChatResponse, StreamResponse};
