use directories::ProjectDirs;
use std::error::Error;
use tokio::fs;

pub struct Info {
    pub db_path: String,
}

pub async fn bootloader() -> Result<Info, Box<dyn Error>> {
    let mut info = Info {
        db_path: String::from("db.sqlite3"),
    };

    if let Some(proj_dirs) = ProjectDirs::from("lk", "levenrok", "mimir") {
        let data_dir = proj_dirs.data_dir();

        let data_dir_exists = fs::try_exists(data_dir).await?;

        if !data_dir_exists {
            fs::create_dir(data_dir).await?;
        }

        let db_path = format!("{}/{}", data_dir.to_str().unwrap(), info.db_path);

        if !fs::try_exists(&db_path).await? {
            fs::File::create(&db_path).await?;
        }

        info.db_path = db_path;
    }

    Ok(info)
}
