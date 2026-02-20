use clap::Parser;

mod database;
use database::Database;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, value_name = "name")]
    create: Option<String>,

    #[arg(short = 'b', long, value_name = "value", requires = "create")]
    shebang: Option<String>,

    #[arg(short, long)]
    list: bool,

    #[arg(short, long, value_name = "name")]
    show: Option<String>,
}

#[tokio::main]
async fn main() {
    _ = Args::parse();

    let conn = Database::open_database("db.sqlite3").await;

    let db = match conn {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    match db.init_database().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    db.close_database().await;
}
