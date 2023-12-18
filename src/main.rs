fn main() {
    if let Err(e) = aoc_rust_{{ year }}::get_args().and_then(aoc_rust_{{ year }}::run) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
