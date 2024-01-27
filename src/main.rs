use std::process;

fn main() {
    if let Err(err) = sway_status::run() {
        eprintln!("Application error: {err:?}");
        process::exit(1);
    }
}
