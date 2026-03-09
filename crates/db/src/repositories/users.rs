//! 用户数据仓库

use sea_orm::*;
use uuid::Uuid;
use crate::models::users;

/// 用户仓库
pub struct UserRepository;

impl UserRepository {
    /// 根据 ID 查找用户
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find_by_id(id)
            .one(db)
            .await
    }
    
    /// 根据用户名和群组查找用户
    pub async fn find_by_username(
        db: &DatabaseConnection,
        group_id: Uuid,
        username: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::GroupId.eq(group_id))
            .filter(users::Column::Username.eq(username))
            .one(db)
            .await
    }
    
    /// 创建用户
    pub async fn create(
        db: &DatabaseConnection,
        user: users::ActiveModel,
    ) -> Result<users::Model, DbErr> {
        user.insert(db).await
    }
    
    /// 更新积分
    pub async fn update_credits(
        db: &DatabaseConnection,
        user_id: Uuid,
        delta: f64,
    ) -> Result<(), DbErr> {
        let user = users::Entity::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".into()))?;
        
        let mut user: users::ActiveModel = user.into();
        user.credits = ActiveValue::Set(user.credits.unwrap() + delta);
        user.update(db).await?;
        
        Ok(())
    }
    
    /// 获取排行榜
    pub async fn leaderboard(
        db: &DatabaseConnection,
        group_id: Uuid,
        limit: u64,
    ) -> Result<Vec<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::GroupId.eq(group_id))
            .order_by_desc(users::Column::Credits)
            .limit(limit)
            .all(db)
            .await
    }
}
