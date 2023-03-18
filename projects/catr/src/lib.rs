use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Ok(file) => {
                read(file, &config);
            }
            Err(e) => eprintln!("Failed to open file {filename}: {}", e)
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => {
            Ok(Box::new(BufReader::new(io::stdin())))
        }
        _ => {
            Ok(Box::new(BufReader::new(File::open(filename)?)))
        }
    }
}

fn read(file: Box<dyn BufRead>, config: &Config) {
    let mut line_no: i32 = 1;
    for line in file.lines() {
        let mut line = line.unwrap();
        if config.number_lines || config.number_nonblank_lines {
            if config.number_nonblank_lines && line.is_empty() {
                println!("{}", line);
                continue;
            }
            line = format!("{:>6}\t{}", line_no, line);
            line_no += 1;
        }
        println!("{}", line);
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Naveen Pantra <naveenpantra.np@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("Files")
                .help("Input files")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("number_lines")
                .value_name("number_lines")
                .short("n")
                .long("number")
                .help("Include line number in output")
                .required(false)
                .takes_value(false)
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .value_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Exclude line numbers for empty lines")
                .required(false)
                .takes_value(false)
                .conflicts_with("number_lines")
        ).get_matches();
    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = if matches.is_present("number_lines") { true } else { false };
    let number_nonblank_lines = if matches.is_present("number_nonblank_lines") { true } else { false };
    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}
