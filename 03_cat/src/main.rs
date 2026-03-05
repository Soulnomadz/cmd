use catr::*;
use clap::Parser;

fn main() {
    if let Err(e) = run(Args::parse()) {
    //if let Err(e) = get_args().and_then(run) {


	eprintln!("{e}");
	std::process::exit(1);
    }
}
