use clap::{Parser, Subcommand};
use std::path::PathBuf;

const DEFAULT_PART_SIZE: u64 = 1024 * 1024 * 50;     //默认分块50M

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Upload(UpOpts),
    Download(DownOpts),
}

#[derive(Debug, Parser)]
pub struct UpOpts {
    #[arg(short='r', long="remote", help="set remote directory")]
    pub remote_dir: String,

    #[arg(short='l', long="local", help="set local backup directory")]
    pub local_dir: String,

    #[arg(
        short='t', 
        long="date", 
        default_value="2",
        value_parser=clap::value_parser!(u8).range(1..=3),
        help="date type for local backup file: 1 => - 2 => _",
    )]
    pub date_type: u8,

    #[arg(short, long, default_value_t=DEFAULT_PART_SIZE)]
    pub part_size: u64,
}

#[derive(Debug, Parser)]
pub struct DownOpts {
    #[arg(value_name="FILENAME")]
    pub filename: String,

    #[arg(short='d', long="local", help="set local directory", default_value=".")]
    pub local_dir: PathBuf,
}
