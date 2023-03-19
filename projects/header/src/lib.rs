use std::error::Error;
use std::io::{Error as IoError, ErrorKind};
use clap::{App, Arg};

const DEFAULT_LINES_OR_BYTES: usize = 10;

struct Config {
    files: Vec<String>,
    lines: Option<usize>,
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() ->  MyResult<()> {
    let matches = App::new("header")
        .about("head cmd of unix")
        .author("Naveen Pantra <naveenpantar.np@gmail.com>")
        .version("0.1.0")
        .arg(
            Arg::with_name("files")
                .value_name("Files")
                .required(false)
                .default_value("-")
                .multiple(true)
        )
        .arg(
            Arg::with_name("lines")
                .takes_value(true)
                .value_name("lines")
                .short("n")
                .long("lines")
                .required(false)
                .help("fist n lines of the file, defaults to 10")
        ).arg(
            Arg::with_name("bytes")
                .value_name("bytes")
                .takes_value(true)
                .short("c")
                .long("bytes")
                .required(false)
                .conflicts_with("lines")
                .help("fist n bytes of the file, defaults to 10")
    ).get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches.value_of("lines");
    let bytes = matches.value_of("bytes");
    let mut config = Config {
        files,
        lines: None,
        bytes: None,
    };
    match get_usize_or_filename(lines) {
        Some((size, None)) => {
            config.lines = size;
        },
        Some((None, filename)) => {
            config.files.push(filename.unwrap());
        },
        None => {
        }
        _ => {},
    }
    dbg!(files, lines, bytes);
    Ok(())
}


fn get_usize_or_filename(args: Option<&str>) -> Option<(Option<usize>, Option<String>)> {
    match args {
        Some(arg) => {
            let arg = arg.parse::<i32>();
            match arg {
                Ok(arg) => {
                    if arg < 0 {
                        return Some((None, None))
                    }
                    Some((Some(arg as usize), None))
                },
                Err(_) => {
                    Some((None, Some(args.unwrap().to_owned())))
                },
            }
        },
        None => {
            Some((None, None))
        }
    }
}
