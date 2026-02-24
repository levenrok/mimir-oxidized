use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};

mod database;
use database::Database;

macro_rules! print_err_exit {
    ($err: expr) => {
        eprintln!("\x1b[33m{}\x1b[0m", $err);
        std::process::exit(1);
    };
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Create a new script
    #[arg(short, long, value_name = "name")]
    create: Option<String>,

    /// Specify a shebang to the new script
    #[arg(short = 'b', long, value_name = "value", requires = "create")]
    shebang: Option<String>,

    /// List all scripts
    #[arg(short, long)]
    list: bool,

    /// Show contents of the specified script
    #[arg(short, long, value_name = "name")]
    show: Option<String>,

    /// Generate autocompletion script
    #[arg(long, value_name = "shell")]
    generate: Option<Shell>,
}

#[tokio::main]
async fn main() {
    let mut cmd = Args::command();
    let args = Args::parse();

    let bin = env!("CARGO_BIN_NAME").to_string();

    if let Some(generator) = args.generate {
        generate(generator, &mut cmd, bin, &mut std::io::stdout());
        return;
    }

    let conn = Database::open_database("db.sqlite3").await;

    let db = match conn {
        Ok(conn) => conn,
        Err(err) => {
            print_err_exit!(err);
        }
    };

    match db.init_database().await {
        Ok(()) => {}
        Err(err) => {
            print_err_exit!(err);
        }
    };

    db.close_database().await;
}
