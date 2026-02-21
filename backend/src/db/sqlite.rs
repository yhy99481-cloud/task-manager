use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::str::FromStr;

pub type DbPool = Pool<Sqlite>;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    // Ensure the directory exists for database files with path
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        let db_path = std::path::Path::new(path);
        // Only create directory if path contains directory separators
        if db_path.parent().is_some() && db_path.parent().unwrap().as_os_str().len() > 0 {
            if let Some(parent) = db_path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
        }
    }

    let pool = SqlitePool::connect(database_url).await?;
    init_database(&pool).await?;
    Ok(pool)
}

async fn init_database(pool: &DbPool) -> Result<()> {
    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create tasks table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create task_status enum for SQLite
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS task_status (
            value TEXT PRIMARY KEY
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Insert status values
    for status in ["todo", "in_progress", "done"] {
        sqlx::query("INSERT OR IGNORE INTO task_status (value) VALUES (?)")
            .bind(status)
            .execute(pool)
            .await?;
    }

    Ok(())
}
