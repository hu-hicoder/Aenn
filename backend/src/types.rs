use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// Article

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct CreateArticle {
    pub title: String,
    pub content: String,
}

// Comment

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub article_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub author: String,
}
