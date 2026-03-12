use anyhow::{Result, anyhow};
use clap::Parser;

use std::io::{BufRead, BufReader, self};
use std::fs::File;

const LONG_ABOUT: &str = r#"
Filter adjacent matching lines from INPUT (or standard input),
writing to OUTPUT (or standard output).

With no options, matching lines are merged to the first occurrence.
"#;

const AFTER_HELP: &str = r#"
A field is a run of blanks (usually spaces and/or TABs), then non-blank
characters.  Fields are skipped before chars.

Note: 'uniq' does not detect repeated lines unless they are adjacent.
You may want to sort the input first, or use 'sort -u' without 'uniq'.

GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
Report any translation bugs to <https://translationproject.org/team/>
Full documentation <https://www.gnu.org/software/coreutils/uniq>
or available locally via: info '(coreutils) uniq invocation'
"#;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=LONG_ABOUT, after_help=AFTER_HELP)]
pub struct Args {
    #[arg(default_value="-")]
    pub in_file: String,

    #[arg(value_name="OUT_FILE")]
    pub out_file: Option<String>,

    #[arg(short, long, help="prefix lines by the number of occurrences")]
    pub count: bool,

    #[arg(short='d', long, help="only print duplicate lines, one for each group")]
    pub repeated: bool,

    #[arg(short, long, help="only print unique lines")]
    pub unique: bool,
}

pub fn run(args: Args) -> Result<()> {
    let mut file = open(&args.in_file).map_err(|e| anyhow!("{}: {}", args.in_file, e))?;
    let mut out_file: Box<dyn io::Write> = match &args.out_file {
	Some(out_name) => Box::new(File::create(out_name)?),
	_ => Box::new(io::stdout()),
    };

    let mut print = |num: u64, text: &str| -> Result<()> {
	if num > 0 {
	    if args.count {
		write!(out_file, "{num:>4} {text}")?;
	    } else {
		write!(out_file, "{text}")?;
	    }
	}
	
	Ok(())
    };

    let mut line_curr = String::new();
    let mut line_prev = String::new();
    let mut count = 0;

    loop {
	let bytes = file.read_line(&mut line_curr)?;
	if bytes == 0 { break; }

	if line_curr.trim_end() !=  line_prev.trim_end() {
	    print(count, &line_prev)?;
	    line_prev = line_curr.clone();
	    count = 0;
	} 
	count += 1;
	line_curr.clear();
    }
    print(count, &line_prev)?;

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
	"-" => Ok(Box::new(BufReader::new(io::stdin()))),
	_   => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
