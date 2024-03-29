use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde_json::json;

use crate::{
    types::{Article, Comment, CreateArticle},
    AppState,
};

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

/// GET /api/articles/{slug}/comments
/// コメントの取得
#[get("/articles/{slug}/comments")]
async fn get_comments(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let slug = path.into_inner().to_string();

    let article_result = sqlx::query_as!(Article, "SELECT * FROM articles WHERE slug = ?", slug)
        .fetch_one(&data.db)
        .await;

    // ToDo:
    match article_result {
        Ok(article) => {
            let comments_result = sqlx::query_as!(
                Comment,
                "SELECT * FROM comments WHERE article_id = ?",
                article.id
            )
            .fetch_all(&data.db)
            .await;

            match comments_result {
                Ok(comments) => HttpResponse::Ok().json(json!(comments)),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("記事が見つかりませんでした。"),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(index)
        .service(get_articles)
        .service(post_article)
        .service(get_article)
        .service(get_comments);
    conf.service(scope);
}
