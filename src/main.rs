mod weather;
use weather::{get_weather_types, get_weather};

fn main() {
    let _ = get_weather_types();
    let _ = get_weather();
}
