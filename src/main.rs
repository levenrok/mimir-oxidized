use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};
use std::process::Command;
use tempfile::NamedTempFile;

mod database;
mod utils;
use database::{Database, Script};
use utils::print_fmt_err;

macro_rules! print_err_exit {
    ($err: expr) => {
        print_fmt_err(&$err.to_string());
        std::process::exit(1);
    };
}

macro_rules! match_or_err_exit {
    ($condition: expr) => {
        match $condition {
            Ok(v) => v,
            Err(e) => {
                print_err_exit!(e);
            }
        }
    };

    ($condition: expr, $fallback: block) => {
        match $condition {
            Ok(v) => v,
            Err(_) => $fallback,
        }
    };
}

macro_rules! print_script {
    ($script: ident) => {
        println!("---");
        println!("\x1b[1mname:\x1b[0m {}", $script.name);
        if let Some(shebang) = &$script.shebang {
            println!("\x1b[1mshebang:\x1b[0m {}", shebang);
        }
        println!("\x1b[1mcontent:\x1b[0m {}", $script.content);
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

    let db = match_or_err_exit!(conn);

    let editor = match_or_err_exit!(std::env::var("EDITOR"), {
        eprintln!("\x1b[33mEDITOR not set. falling back to 'nano'...\x1b[0m");
        String::from("nano")
    });

    match_or_err_exit!(db.init_database().await);

    if let Some(name) = args.create {
        let script = match_or_err_exit!(db.select_script(&name).await);
        match script {
            Some(_) => {
                db.close_database().await;
                print_err_exit!(format!("Script named '{}' already exists!", name));
            }
            None => {}
        }

        let path = match NamedTempFile::new() {
            Ok(temp) => String::from(temp.path().to_str().unwrap()),
            Err(err) => {
                db.close_database().await;
                print_err_exit!(err);
            }
        };

        let mut child = Command::new(editor)
            .arg(&path)
            .spawn()
            .expect("error: failed to spawn child process!");
        match_or_err_exit!(child.wait());

        let content =
            match_or_err_exit!(tokio::fs::read_to_string(path).await, { String::from("") });

        let script = Script {
            name: name.trim().to_string(),
            content: content.trim().to_string(),
            shebang: args.shebang,
        };

        match db.insert_script(&script).await {
            Ok(()) => {
                print_script!(script);
            }
            Err(err) => {
                db.close_database().await;
                print_err_exit!(err);
            }
        }
    }

    if args.list {
        let scripts = db.select_scripts().await;

        match scripts {
            Ok(rows) => {
                rows.iter().for_each(|script| {
                    print_script!(script);
                    println!();
                });
            }
            Err(err) => {
                db.close_database().await;
                print_err_exit!(err);
            }
        }
    }

    if let Some(name) = args.show {
        let script = match_or_err_exit!(db.select_script(&name).await);
        match script {
            Some(script) => {
                print_script!(script);
            }
            None => {
                db.close_database().await;
                print_err_exit!(format!("No script named '{}' found!", name));
            }
        }
    }

    db.close_database().await;
}
