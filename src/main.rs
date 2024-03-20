mod icons;
mod weather;
use icons::get_icon;
use weather::run;

fn main() {
    // if let Err(err) = run() {
    //     eprintln!("Error: {}", err);
    // }

    let icon_unknown = get_icon("iconUnknown");
    for line in &icon_unknown {
        println!("{}", line);
    }
}
