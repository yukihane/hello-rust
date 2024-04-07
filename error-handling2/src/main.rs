use error_handling2::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
