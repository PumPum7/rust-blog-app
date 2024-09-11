use crate::errors::AppError;
use rusqlite::Connection;
use std::sync::Mutex;

pub struct AppState {
    pub db: Mutex<Connection>,
}

impl AppState {
    pub fn new() -> Result<Self, AppError> {
        let db = Connection::open("blogposts.db")?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS blogposts (
                id TEXT PRIMARY KEY,
                text TEXT NOT NULL,
                date TEXT NOT NULL,
                image TEXT,
                username TEXT NOT NULL,
                avatar TEXT
            )",
            [],
        )?;

        Ok(Self {
            db: Mutex::new(db),
        })
    }
}
