use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
