//! 使用日志数据仓库

use sea_orm::*;
use uuid::Uuid;
use crate::models::usage_logs;

/// 使用日志仓库
pub struct UsageRepository;

impl UsageRepository {
    /// 创建使用日志
    pub async fn create(
        db: &DatabaseConnection,
        log: usage_logs::ActiveModel,
    ) -> Result<usage_logs::Model, DbErr> {
        log.insert(db).await
    }
    
    /// 获取用户使用记录
    pub async fn list_by_user(
        db: &DatabaseConnection,
        user_id: Uuid,
        limit: u64,
    ) -> Result<Vec<usage_logs::Model>, DbErr> {
        usage_logs::Entity::find()
            .filter(usage_logs::Column::UserId.eq(user_id))
            .order_by_desc(usage_logs::Column::CreatedAt)
            .limit(limit)
            .all(db)
            .await
    }
}

/// 使用统计摘要
#[derive(Debug, Clone)]
pub struct UsageSummary {
    pub total_requests: u64,
    pub total_tokens: i64,
    pub total_cost: f64,
}
