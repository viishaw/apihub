//! 数据模型

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// ============ Groups ============

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub owner_id: Option<Uuid>,
    pub invite_code: String,
    pub settings: Json,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// ============ Users ============

pub mod users {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub group_id: Uuid,
        pub username: String,
        pub email: String,
        pub password_hash: String,
        pub credits: f64,
        pub is_admin: bool,
        pub is_active: bool,
        pub settings: Json,
        pub last_active_at: Option<DateTimeUtc>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ============ API Keys ============

pub mod api_keys {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "api_keys")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub group_id: Uuid,
        pub contributor_id: Uuid,
        pub provider: String,
        pub name: Option<String>,
        pub encrypted_key: Vec<u8>,
        pub key_hash: String,
        pub base_url: Option<String>,
        pub monthly_quota: Option<f64>,
        pub used_quota: f64,
        pub weight: i32,
        pub is_active: bool,
        pub last_used_at: Option<DateTimeUtc>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ============ Usage Logs ============

pub mod usage_logs {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "usage_logs")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub group_id: Uuid,
        pub user_id: Uuid,
        pub key_id: Uuid,
        pub provider: String,
        pub model: String,
        pub input_tokens: Option<i32>,
        pub output_tokens: Option<i32>,
        pub total_tokens: Option<i32>,
        pub cost_usd: Option<f64>,
        pub credits_earned: Option<f64>,
        pub latency_ms: Option<i32>,
        pub status_code: Option<i32>,
        pub error_message: Option<String>,
        pub created_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ============ Credit Transactions ============

pub mod credit_transactions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "credit_transactions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub group_id: Uuid,
        pub user_id: Uuid,
        pub amount: f64,
        #[sea_orm(column_type = "Text")]
        pub transaction_type: String,
        pub reason: Option<String>,
        pub related_log_id: Option<Uuid>,
        pub created_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
