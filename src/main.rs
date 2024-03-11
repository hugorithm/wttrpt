mod weather;
use weather::{get_weather, get_weather_types};

fn main() {
    match get_weather_types() {
        Ok(weather_types) => {
            println!("Weather types: {:?}", weather_types);
        }
        Err(err) => {
            eprintln!("Error getting weather types: {}", err);
        }
    }

    match get_weather() {
        Ok(weather) => {
            for wttr in &weather {
                println!("{:?}", wttr);
            }
        }
        Err(err) => {
            eprintln!("Error getting weather data: {}", err);
        }
    }
}
