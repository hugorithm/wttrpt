mod weather;
use weather::{get_weather, get_weather_types};

fn main() {
    let weather_types = get_weather_types();
    let weather = get_weather();
}
