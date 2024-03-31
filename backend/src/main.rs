use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;

mod articles;
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

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"]) // "PATCH", "DELETE"
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(articles::config)
            .wrap(cors)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
