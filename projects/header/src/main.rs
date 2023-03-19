fn main() {
    if let Err(e) = header::get_args() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
