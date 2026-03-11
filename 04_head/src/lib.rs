use anyhow::{Result, anyhow};
use clap::Parser;
use std::io::{BufRead, BufReader, self};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "LINES", default_value = "10", short = 'n', long, value_parser = clap::value_parser!(u64).range(1..))]
    pub lines: u64,

    #[arg(value_name = "BYTES", short = 'c', long, conflicts_with("lines"), value_parser = clap::value_parser!(u64).range(1..1024))]
    pub bytes: Option<u64>,

    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,
}

fn open(file: &str) -> Result<Box<dyn BufRead>> {
    match file {
	"-" => Ok(Box::new(BufReader::new(io::stdin()))),
	_   => Ok(Box::new(BufReader::new(std::fs::File::open(file)?))),
    }
}

pub fn run(args: Args) -> Result<()> {
    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
	match open(&filename) {
	    Ok(mut f) => { 
		if num_files > 1 {
		    println!(
			"{}==> {filename} <==",
			if file_num > 0 { "\n" } else { "" },  //第一个文件无须在上方添加空行
		    );
		}

		if let Some(num_bytes) = args.bytes {
		    let mut buffer = vec![0; num_bytes as usize];
		    let bytes_read = f.read(&mut buffer)?;
		    
		    print!(
			"{}",
			String::from_utf8_lossy(&buffer[..bytes_read])
		    );
		} else {
		    for line in f.lines().take(args.lines as usize) {
		        println!("{}", line?);
		    }
		}
	    }
	    Err(e) => return Err(anyhow!("failed to open {}: {}", filename, e)),
	}
    }

    Ok(())
}

/*
fn parse_positive_int(val: &str) -> Result<u64> {
    match val.parse() {
	Ok(n) if n > 0 => Ok(n),
	_              => Err(anyhow!("{}", val)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_positive_int() {
	// 3 is ok
 	let result = parse_positive_int("3");
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), 3);

	// any string is an error
	let result = parse_positive_int("foo");
	assert!(result.is_err());
	assert_eq!(result.unwrap_err().to_string(), "foo".to_string());

	// 0 is an error
	let result = parse_positive_int("0");
	assert!(result.is_err());
	assert_eq!(result.unwrap_err().to_string(), "0".to_string());
    }
}

*/
