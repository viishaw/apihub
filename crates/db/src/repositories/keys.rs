//! API Key 数据仓库

use sea_orm::*;
use uuid::Uuid;
use crate::models::api_keys;

/// Key 仓库
pub struct KeyRepository;

impl KeyRepository {
    /// 根据 ID 查找 Key
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<api_keys::Model>, DbErr> {
        api_keys::Entity::find_by_id(id)
            .one(db)
            .await
    }
    
    /// 根据哈希查找 Key（用于去重）
    pub async fn find_by_hash(
        db: &DatabaseConnection,
        group_id: Uuid,
        key_hash: &str,
    ) -> Result<Option<api_keys::Model>, DbErr> {
        api_keys::Entity::find()
            .filter(api_keys::Column::GroupId.eq(group_id))
            .filter(api_keys::Column::KeyHash.eq(key_hash))
            .one(db)
            .await
    }
    
    /// 创建 Key
    pub async fn create(
        db: &DatabaseConnection,
        key: api_keys::ActiveModel,
    ) -> Result<api_keys::Model, DbErr> {
        key.insert(db).await
    }
    
    /// 删除 Key
    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        api_keys::Entity::delete_by_id(id)
            .exec(db)
            .await?;
        Ok(())
    }
    
    /// 获取群组内所有 Key
    pub async fn list_by_group(
        db: &DatabaseConnection,
        group_id: Uuid,
    ) -> Result<Vec<api_keys::Model>, DbErr> {
        api_keys::Entity::find()
            .filter(api_keys::Column::GroupId.eq(group_id))
            .all(db)
            .await
    }
    
    /// 获取用户贡献的 Key
    pub async fn list_by_contributor(
        db: &DatabaseConnection,
        contributor_id: Uuid,
    ) -> Result<Vec<api_keys::Model>, DbErr> {
        api_keys::Entity::find()
            .filter(api_keys::Column::ContributorId.eq(contributor_id))
            .all(db)
            .await
    }
    
    /// 获取可用的 Key（按提供商过滤）
    pub async fn list_available(
        db: &DatabaseConnection,
        group_id: Uuid,
        provider: &str,
    ) -> Result<Vec<api_keys::Model>, DbErr> {
        api_keys::Entity::find()
            .filter(api_keys::Column::GroupId.eq(group_id))
            .filter(api_keys::Column::Provider.eq(provider))
            .filter(api_keys::Column::IsActive.eq(true))
            .all(db)
            .await
    }
    
    /// 更新使用量
    pub async fn update_usage(
        db: &DatabaseConnection,
        key_id: Uuid,
        additional_quota: f64,
    ) -> Result<(), DbErr> {
        let key = api_keys::Entity::find_by_id(key_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Key not found".into()))?;
        
        let mut key: api_keys::ActiveModel = key.into();
        key.used_quota = ActiveValue::Set(key.used_quota.unwrap() + additional_quota);
        key.last_used_at = ActiveValue::Set(Some(chrono::Utc::now()));
        key.update(db).await?;
        
        Ok(())
    }
    
    /// 切换 Key 状态
    pub async fn toggle_active(
        db: &DatabaseConnection,
        key_id: Uuid,
    ) -> Result<bool, DbErr> {
        let key = api_keys::Entity::find_by_id(key_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Key not found".into()))?;
        
        let new_status = !key.is_active;
        let mut key: api_keys::ActiveModel = key.into();
        key.is_active = ActiveValue::Set(new_status);
        key.update(db).await?;
        
        Ok(new_status)
    }
}
