use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::get;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Weather {
    tMin: Option<String>,
    tMax: Option<String>,
    tMed: Option<String>,
    idFfxVento: Option<i32>,
    iUv: Option<String>,
    idTipoTempo: Option<i32>,
    globalIdLocal: Option<i32>,
    probabilidadePrecipita: Option<String>,
    idPeriodo: Option<i32>,
    dataPrev: Option<String>,
    ddVento: Option<String>,
}

// Weather Types: https://www.ipma.pt/bin/file.data/weathertypes.json

fn main() -> Result<(), Box<dyn Error>> {
    let resp = match reqwest::blocking::get("https://api.ipma.pt/public-data/forecast/aggregate/1030300.json") {
        Ok(resp) => resp,
        Err(err) => {
            println!("Error: {}", err);
            return Ok(());
        }
    };

    if resp.status().is_success() {
        let body = match resp.text() {
            Ok(body) => body,
            Err(err) => {
                println!("Error reading response body: {}", err);
                return Ok(());
            }
        };

        let weather_data: Vec<Weather> = match serde_json::from_str(&body) {
            Ok(entries) => entries,
            Err(err) => {
                println!("Error parsing JSON: {}", err);
                return Ok(());
            }
        };

        for entry in weather_data {
            println!("{:?}", entry);
        }
    } else {
        println!("Error: {}", resp.status());
    }

    Ok(())
}
