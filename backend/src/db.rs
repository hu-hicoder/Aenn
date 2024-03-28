use actix_web::{get, web, HttpResponse, Responder};
use sqlx::mysql::MySql;
use sqlx::Pool;

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

#[get("/hey")]
pub async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(hey).service(index);
    conf.service(scope);
}
