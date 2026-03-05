use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(args: Args) -> MyResult<()> {
    for f in args.files {
	match open(&f) {
	    Err(e) => eprintln!("Failed to open {f}: {e}"),
	    Ok(file) => {
		// 记录行号
	 	let mut line_num = 0;
		// 记录上一行是否空行
		let mut prev_blank = false;

		for line_result in file.lines() {
		    let line = line_result?;
		    let is_blank = line.is_empty();

		    // 处理参数-s，压缩多个空行为一个
		    if args.squeeze_blank && is_blank && prev_blank { continue }

		    // 处理参数-n，每行前面打印行号
		    // 如果同时传入-s，因为前面已经压缩了空行，此处直接输出即可
		    if args.number_lines {
		    	line_num += 1;
			println!("{:>6}\t{}", line_num, line);
		    // 处理参数-b，仅在非空行前输出行号
		    } else if args.number_nonblank_lines && !is_blank {
		    	line_num += 1;
			println!("{:>6}\t{}", line_num, line);
		    // 处理无参数
		    // 传入参数-b时空行同样直接输出即可
		    } else {
		        println!("{line}");
		    }

	    	    prev_blank = is_blank;
		}

	    }
	}
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,

    #[arg(
	short('n'),
	long("number"),
	conflicts_with("number_nonblank_lines")	
    )]
    pub number_lines: bool,

    #[arg(short('b'), long("number-nonblank"))]
    pub number_nonblank_lines: bool,

    #[arg(short('s'), long("squeeze-blank"))]
    pub squeeze_blank: bool,
}

pub fn get_args() -> MyResult<Args> {
    Ok(Args::parse())
}

fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
 	"-" => Ok(Box::new(BufReader::new(io::stdin()))),
	_   => Ok(Box::new(BufReader::new(File::open(file)?))), 
    }
}

