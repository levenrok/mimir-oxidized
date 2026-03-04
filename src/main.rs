use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};
use std::process::Command;

mod database;
use database::{Database, Script};
use tempfile::NamedTempFile;

macro_rules! print_err_exit {
    ($err: expr) => {
        eprintln!("\x1b[31m{}\x1b[0m", $err);
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
        match db.select_script(&name).await {
            Ok(script) => match script {
                Some(_) => {
                    print_err_exit!(format!("Script named '{}' already exists!", name));
                }
                None => {}
            },
            Err(_) => {}
        }

        let path = match NamedTempFile::new() {
            Ok(temp) => String::from(temp.path().to_str().unwrap()),
            Err(err) => {
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
                print_err_exit!(err);
            }
        }
    }

    db.close_database().await;
}
