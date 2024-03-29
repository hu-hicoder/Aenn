use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;

mod db;
mod types;

pub struct AppState {
    db: MySqlPool,
}

// http://localhost:8080/api/articles

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLを設定してください");
    let pool = match MySqlPoolOptions::new()
        .max_connections(7)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("⭕ データベースに接続しました");
            pool
        }
        Err(err) => {
            println!("⛔ データベースに接続できませんでした: {:?}", err);
            std::process::exit(1); // 異常終了
        }
    };
    println!("🖥 サーバーが起動しました。");

    db::init_db(&pool)
        .await
        .expect("⛔ データベースの初期化に失敗しました");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(db::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
