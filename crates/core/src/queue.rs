//! 排队模块

use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

/// 排队请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueRequest {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub model: String,
    pub priority: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: QueueStatus,
}

/// 排队状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// 排队管理器
pub struct QueueManager {
    redis: redis::Client,
}

impl QueueManager {
    pub fn new(redis: redis::Client) -> Self {
        Self { redis }
    }
    
    /// 加入队列
    pub async fn enqueue(&self, group_id: uuid::Uuid, req: QueueRequest) -> Result<(), crate::error::Error> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;
        let key = format!("apihub:queue:{}", group_id);
        let score = req.priority;
        let member = serde_json::to_string(&req)?;
        
        let _: () = conn.zadd(&key, member, score).await?;
        Ok(())
    }
    
    /// 获取下一个请求（最高优先级）
    pub async fn dequeue(&self, group_id: uuid::Uuid) -> crate::error::Result<Option<QueueRequest>> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;
        let key = format!("apihub:queue:{}", group_id);
        
        // 使用 ZPOPMAX 获取最高分数的元素
        let result: Option<Vec<String>> = redis::cmd("ZPOPMAX")
            .arg(&key)
            .arg(1)
            .query_async(&mut conn)
            .await?;
        
        match result {
            Some(mut members) if !members.is_empty() => {
                let json = members.remove(0);
                let req: QueueRequest = serde_json::from_str(&json)?;
                Ok(Some(req))
            }
            _ => Ok(None),
        }
    }
    
    /// 获取队列长度
    pub async fn length(&self, group_id: uuid::Uuid) -> crate::error::Result<u64> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;
        let key = format!("apihub:queue:{}", group_id);
        
        let len = conn.zcard(&key).await?;
        Ok(len)
    }
    
    /// 获取用户在队列中的位置
    pub async fn position(&self, group_id: uuid::Uuid, user_id: uuid::Uuid) -> crate::error::Result<Option<u64>> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;
        let key = format!("apihub:queue:{}", group_id);
        
        // 获取所有请求
        let members: Vec<String> = conn.zrange(&key, 0, -1).await?;
        
        // 查找用户位置
        for (idx, member) in members.iter().enumerate() {
            let req: QueueRequest = serde_json::from_str(member)?;
            if req.user_id == user_id {
                return Ok(Some(idx as u64 + 1));
            }
        }
        
        Ok(None)
    }
}
