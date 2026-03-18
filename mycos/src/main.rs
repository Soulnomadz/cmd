use clap::Parser;
use mycos::*;

#[tokio::main]
async fn main() {
    if let Err(e) = run(Cli::parse()).await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
