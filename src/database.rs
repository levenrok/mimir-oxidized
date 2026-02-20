use sqlx::{Pool, Sqlite, sqlite};
use std::error::Error;

pub struct Database {
    db: Pool<Sqlite>,
    path: String,
}

impl Database {
    pub async fn open_database(path: &str) -> Result<Self, Box<dyn Error>> {
        let db_exists = tokio::fs::try_exists(path).await?;
        if !db_exists {
            tokio::fs::File::create(path).await?;
        }

        let url = format!("sqlite:{}", path);
        let pool = sqlite::SqlitePool::connect(&url).await?;

        Ok(Database {
            db: pool,
            path: String::from(path),
        })
    }

    pub async fn init_database(&self) -> Result<(), Box<dyn Error>> {
        sqlx::query("PRAGMA journal_mode = WAL;")
            .execute(&self.db)
            .await?;

        sqlx::migrate!().run(&self.db).await?;

        Ok(())
    }

    pub async fn close_database(&self) -> () {
        self.db.close().await
    }
}
