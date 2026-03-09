//! 调度器模块

use async_trait::async_trait;

/// API Key 信息
#[derive(Debug, Clone)]
pub struct KeyInfo {
    pub id: uuid::Uuid,
    pub provider: String,
    pub encrypted_key: Vec<u8>,
    pub base_url: Option<String>,
    pub weight: i32,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 调度器 trait
#[async_trait]
pub trait Scheduler: Send + Sync {
    /// 选择一个可用的 Key
    async fn select_key(&self, provider: &str, model: &str) -> crate::error::Result<KeyInfo>;
    
    /// 报告 Key 使用
    async fn report_usage(&self, key_id: uuid::Uuid, tokens: u32) -> crate::error::Result<()>;
    
    /// 报告 Key 错误
    async fn report_error(&self, key_id: uuid::Uuid, error: &str) -> crate::error::Result<()>;
}

/// 轮询调度器
pub struct RoundRobinScheduler {
    // TODO: 实现轮询逻辑
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Scheduler for RoundRobinScheduler {
    async fn select_key(&self, provider: &str, model: &str) -> crate::error::Result<KeyInfo> {
        todo!()
    }
    
    async fn report_usage(&self, key_id: uuid::Uuid, tokens: u32) -> crate::error::Result<()> {
        todo!()
    }
    
    async fn report_error(&self, key_id: uuid::Uuid, error: &str) -> crate::error::Result<()> {
        todo!()
    }
}

/// 权重调度器
pub struct WeightedScheduler {
    // TODO: 实现权重逻辑
}

impl WeightedScheduler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Scheduler for WeightedScheduler {
    async fn select_key(&self, provider: &str, model: &str) -> crate::error::Result<KeyInfo> {
        todo!()
    }
    
    async fn report_usage(&self, key_id: uuid::Uuid, tokens: u32) -> crate::error::Result<()> {
        todo!()
    }
    
    async fn report_error(&self, key_id: uuid::Uuid, error: &str) -> crate::error::Result<()> {
        todo!()
    }
}
