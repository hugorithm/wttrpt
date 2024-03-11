mod weather;
use weather::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
}
