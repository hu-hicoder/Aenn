use sqlx::mysql::MySqlPool;

pub async fn add_init(pool: &MySqlPool) -> anyhow::Result<()> {
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
