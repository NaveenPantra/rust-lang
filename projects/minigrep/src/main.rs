use std::env;
use std::process;

use minigrep::Config;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("{err}");
    process::exit(1);
  });
  if let Err(err) = minigrep::run(config) {
    eprintln!("Application Error: {}", err);
    process::exit(1);
  }
}