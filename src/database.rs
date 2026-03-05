use sqlx::{Pool, Sqlite, SqlitePool};
use std::error::Error;

pub struct Database {
    db: Pool<Sqlite>,
}

pub struct Script {
    pub name: String,
    pub content: String,
    pub shebang: Option<String>,
}

impl Database {
    pub async fn open_database(path: &str) -> Result<Self, Box<dyn Error>> {
        let db_exists = tokio::fs::try_exists(path).await?;
        if !db_exists {
            tokio::fs::File::create(path).await?;
        }

        let url = format!("sqlite:{}", path);
        let pool = SqlitePool::connect(&url).await?;

        Ok(Database { db: pool })
    }

    pub async fn init_database(&self) -> Result<(), sqlx::Error> {
        sqlx::query("PRAGMA journal_mode = WAL;")
            .execute(&self.db)
            .await?;

        sqlx::migrate!().run(&self.db).await?;

        Ok(())
    }

    pub async fn insert_script(&self, script: &Script) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO scripts (name, content, shebang) VALUES ($1, $2, $3);",
            script.name,
            script.content,
            script.shebang
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn select_script(&self, name: &str) -> Result<Option<Script>, sqlx::Error> {
        let script = sqlx::query_as!(
            Script,
            r#"SELECT name, content as "content!", shebang FROM scripts WHERE name = $1;"#,
            name
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(script)
    }

    pub async fn select_scripts(&self) -> Result<Vec<Script>, sqlx::Error> {
        let scripts = sqlx::query_as!(
            Script,
            r#"SELECT name, content as "content!", shebang FROM scripts;"#,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(scripts)
    }

    pub async fn close_database(&self) -> () {
        self.db.close().await
    }
}
