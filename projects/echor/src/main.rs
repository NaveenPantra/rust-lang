use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Naveen Pantra <naveenpantra.np@gmail.com>")
        .about("Rust version of echo cmd")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false)
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();

    print!("{}{}", text.join(" "), if matches.is_present("omit_newline") { "" } else { "\n" });
}
