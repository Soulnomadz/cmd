use wc::*;
use clap::Parser;

fn main() {
    if let Err(e) = run(Args::parse()) {
	eprintln!("{}", e);
	std::process::exit(1);
    }
}
