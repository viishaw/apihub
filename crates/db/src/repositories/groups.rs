//! 群组数据仓库

use sea_orm::*;
use uuid::Uuid;
use crate::models as all_models;

/// 群组仓库
pub struct GroupRepository;

impl GroupRepository {
    /// 根据 ID 查找群组
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<all_models::Model>, DbErr> {
        all_models::Entity::find_by_id(id)
            .one(db)
            .await
    }
    
    /// 根据邀请码查找群组
    pub async fn find_by_invite_code(
        db: &DatabaseConnection,
        invite_code: &str,
    ) -> Result<Option<all_models::Model>, DbErr> {
        all_models::Entity::find()
            .filter(all_models::Column::InviteCode.eq(invite_code))
            .one(db)
            .await
    }
    
    /// 创建群组
    pub async fn create(
        db: &DatabaseConnection,
        group: all_models::ActiveModel,
    ) -> Result<all_models::Model, DbErr> {
        group.insert(db).await
    }
    
    /// 重新生成邀请码
    pub async fn regenerate_invite_code(
        db: &DatabaseConnection,
        group_id: Uuid,
        new_code: String,
    ) -> Result<(), DbErr> {
        let group = all_models::Entity::find_by_id(group_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Group not found".into()))?;
        
        let mut group: all_models::ActiveModel = group.into();
        group.invite_code = ActiveValue::Set(new_code);
        group.update(db).await?;
        
        Ok(())
    }
}
