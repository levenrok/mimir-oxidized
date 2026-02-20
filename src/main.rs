use clap::Parser;

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
}
