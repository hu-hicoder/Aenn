use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde_json::json;
use sqlx::mysql::MySql;
use sqlx::Pool;

use crate::{
    types::{Article, CreateArticle},
    AppState,
};

/// データベースの初期化
pub async fn init_db(pool: &Pool<MySql>) -> anyhow::Result<()> {
    // Create articles table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS articles (
            id INT AUTO_INCREMENT PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            slug VARCHAR(255) NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create author table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS author (
            name VARCHAR(255) NOT NULL,
            avatarUrl VARCHAR(255) NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create comments table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS comments (
            id INT AUTO_INCREMENT PRIMARY KEY,
            body TEXT NOT NULL,
            article_id INT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            author VARCHAR(255) NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// GET /api/articles
/// 記事一覧を取得する
/// 型: Article
#[get("/articles")]
async fn get_articles(data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(Article, "SELECT * FROM articles")
        .fetch_all(&data.db)
        .await
    {
        Ok(articles) => HttpResponse::Ok().json(json!({"articles": articles})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// POST /api/articles
/// 新規記事を作成する
/// テスト:
/// ``
/// curl -X POST http://localhost:8080/api/articles \
///      -H "Content-Type: application/json" \
///      -d '{"title": "記事のタイトル", "content": "記事の内容"}'
/// ``
#[post("/articles")]
async fn post_article(body: web::Json<CreateArticle>, data: web::Data<AppState>) -> impl Responder {
    let slug = uuid::Uuid::new_v4().to_string();
    let article = body.into_inner();
    let now: NaiveDateTime = chrono::Utc::now().naive_utc();

    let result = sqlx::query(
        r#"
        INSERT INTO articles (title, content, slug, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(&article.title)
    .bind(&article.content)
    .bind(&slug)
    .bind(&now)
    .bind(&now)
    .execute(&data.db)
    .await;

    // ToDo: 成功、失敗時のjsonのレスポンスを返すようにする
    match result {
        Ok(_) => HttpResponse::Created().json(json!({"slug": slug})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// GET /api/articles/{slug}
/// 特定の記事を取得する
#[get("/articles/{slug}")]
async fn get_article(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let slug = path.into_inner().to_string();
    let result = sqlx::query_as!(Article, "SELECT * FROM articles WHERE slug = ?", slug)
        .fetch_one(&data.db)
        .await;

    match result {
        Ok(article) => {
            // println!("{:?}", json!(article));
            HttpResponse::Ok().json(json!(article))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(index)
        .service(get_articles)
        .service(post_article)
        .service(get_article);
    conf.service(scope);
}
