use crate::errors::AppError;
use sqlx::sqlite::SqlitePool;
use std::fs::File;
use std::path::Path;

pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub async fn new() -> Result<Self, AppError> {
        let db_path = "sqlite:blogposts.db";

        // Check if the database file exists, if not, create it
        if !Path::new("blogposts.db").exists() {
            File::create("blogposts.db").expect("Failed to create database file");
        }

        let db = SqlitePool::connect(db_path)
            .await
            .expect("Failed to connect to database");
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS blogposts (
                id TEXT PRIMARY KEY,
                text TEXT NOT NULL,
                date TEXT NOT NULL,
                image TEXT,
                username TEXT NOT NULL,
                avatar TEXT
            )",
        )
        .execute(&db)
        .await?;

        Ok(Self { db })
    }
}
