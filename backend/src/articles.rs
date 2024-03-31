use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::mysql::MySqlPool;

use crate::AppState;

/// GET /api/articles
/// 記事一覧を取得する
#[get("/articles")]
async fn get_articles(data: web::Data<AppState>) -> impl Responder {
    #[derive(Debug, Serialize)]
    struct GetArticle {
        id: i32,
        title: String,
        content: String,
        slug: String,
        user_id: i32,
    }

    // ToDo 最新記事を全部返すのは多すぎる
    // ToDo エラーハンドリング
    match sqlx::query_as!(
        GetArticle,
        "SELECT id, title, content, slug, user_id FROM articles"
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(articles) => HttpResponse::Ok().json(json!(articles)),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error" : err.to_string()})),
    }
}

/// 記事を追加して、その時の記事のIDを返す
async fn add_article(
    pool: &MySqlPool,
    title: String,
    content: String,
    user_id: i32,
) -> anyhow::Result<u64> {
    let last_article_id = sqlx::query!(
        r#"
        INSERT INTO articles (title, content, slug, user_id)
        VALUES (?, ?, UUID(), ?);
        "#,
        title,
        content,
        user_id
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(last_article_id)
}

#[derive(Debug, Deserialize)]
// Todo subcategory_idの機能追加
struct CreateArticle {
    pub title: String,
    pub content: String,
    pub subcategory_id: i32,
}

/// POST /api/articles
/// 新規記事を作成する
#[post("/articles")]
async fn post_article(body: web::Json<CreateArticle>, data: web::Data<AppState>) -> impl Responder {
    let article = body.into_inner();
    // ToDo ユーザーIDの機能の追加 (fix 1)
    const USER_ID: i32 = 1;

    let article_id = match add_article(&data.db, article.title, article.content, USER_ID).await {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    };

    // ToDo サブカテゴリーの追加
    let subcategory_ids = vec![1, 2, 3]; // 仮のサブカテゴリーID
    for subcategory_id in subcategory_ids {
        match sqlx::query!(
            r#"
            INSERT INTO article_subcategory (article_id, subcategory_id)
            VALUES (?, ?);
            "#,
            article_id,
            subcategory_id
        )
        .execute(&data.db)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                return HttpResponse::InternalServerError().json(json!({"error" : err.to_string()}))
            }
        }
    }

    HttpResponse::Ok().json(json!({"messege": "success"}))

    // ToDo: 成功、失敗時のjsonのレスポンスを返すようにする
}

/// GET /api/articles/{slug}
/// 特定の記事とコメントを取得する
#[get("/articles/{slug}")]
async fn get_article(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let slug = path.into_inner().to_string();

    #[derive(Debug, Serialize)]
    struct GetArticle {
        id: i32,
        title: String,
        content: String,
        slug: String,
        user_id: i32,
    }
    let article = match sqlx::query_as!(
        GetArticle,
        "SELECT id, title, content, slug, user_id FROM articles WHERE slug = ?",
        slug
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(article) => article,
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({"error" : err.to_string()}));
        }
    };

    #[derive(Debug, Serialize)]
    struct GetComment {
        id: i32,
        content: String,
        user_id: i32,
        article_id: i32,
        created_at: chrono::NaiveDateTime,
    }
    let comments = match sqlx::query_as!(
        GetComment,
        "SELECT id, content, user_id, article_id, created_at FROM comments WHERE article_id = ?",
        article.id
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(comments) => comments,
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({"error" : err.to_string()}));
        }
    };

    HttpResponse::Ok().json(json!({
        "article": {
            "title": article.title,
            "content": article.content,
            "user_id": article.user_id, // ToDo: ユーザー名に変更
        },
        "comments": comments.iter().map(|comment| {
            json!({
                "content": comment.content,
                "user_id": comment.user_id, //　ToDo: ユーザー名に変更
            })
        }).collect::<Vec<serde_json::Value>>()
    }))
    // "user_id": comment.user_id,
    // "created_at": comment.created_at
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_articles)
        .service(post_article)
        .service(get_article);
    // .service(get_comments);
    conf.service(scope);
}
