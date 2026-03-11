use anyhow::Result;
use clap::Parser;

use std::io::{BufRead, BufReader, self};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, help="print the newline counts")]
    pub lines: bool,

    #[arg(short, long, help="print the word counts")]
    pub words: bool,

    #[arg(short='m', long, help="print the character counts")]
    pub chars: bool,

    #[arg(short='c', long, conflicts_with("chars"), help="print the byte counts")]
    pub bytes: bool,

    #[arg(value_name="FILE", default_value="-")]
    files: Vec<String>,
}

fn open(file: &str) -> Result<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _   => Ok(Box::new(BufReader::new(std::fs::File::open(file)?))),
    }
}

pub fn run(mut args: Args) -> Result<()> {
    if [args.lines, args.words, args.chars, args.bytes].iter().all(|v| v == &false) {
	args.lines = true;
	args.words = true;
	args.bytes = true;
    }

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &args.files {
	match open(filename) {
	    Err(e) => eprintln!("{}: {}", filename, e),
	    Ok(file)  => { 
		if let Ok(info) = count(file) {
		    println!(
		        "{}{}{}{}{}",
		        format_field(info.num_lines, args.lines),
		        format_field(info.num_words, args.words),
		        format_field(info.num_bytes, args.bytes),
		        format_field(info.num_chars, args.chars),
			if filename == "-" { format!("") } else { format!(" {filename}") }
		    );

		    total_lines += info.num_lines;
		    total_words += info.num_words;
		    total_bytes += info.num_bytes;
		    total_chars += info.num_chars;
		}
	    }
	}
    }

    if args.files.len() > 1 {
   	 println!(
   	     "{}{}{}{} total",
   	     format_field(total_lines, args.lines),
   	     format_field(total_words, args.words),
   	     format_field(total_bytes, args.bytes),
   	     format_field(total_chars, args.chars),
   	 );
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
pub(crate) struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub(crate) fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();

    loop {
	let line_bytes = file.read_line(&mut line)?;
	if line_bytes == 0 {
	    break;
	}

	num_lines += 1;
	num_words += line.split_whitespace().count();
	num_bytes += line_bytes;
	num_chars += line.chars().count();

	line.clear();
    }

    Ok(FileInfo {
	num_lines,
	num_words,
	num_bytes,
	num_chars,
    })
}

pub(crate) fn format_field(value: usize, show: bool) -> String {
    if show {
	format!("{value:>8}")
    } else {
	"".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo, format_field};
    use std::io::Cursor;

    #[test]
    fn test_count() {
	let text = "I don't want the world. I just want your half.\r\n";
	let info = count(Cursor::new(text));

	assert!(info.is_ok());

	let expected = FileInfo {
	    num_lines: 1,
	    num_words: 10,
	    num_chars: 48,
	    num_bytes: 48,
	};
	assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
	assert_eq!(format_field(1, false), "");
	assert_eq!(format_field(3, true), "       3");
	assert_eq!(format_field(10, true), "      10");
    }
}
