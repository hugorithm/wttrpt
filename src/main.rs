use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::get;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Weather {
    #[serde(rename = "tMin")]
    t_min: Option<String>,
    #[serde(rename = "tMax")]
    t_max: Option<String>,
    #[serde(rename = "tMed")]
    t_med: Option<String>,
    #[serde(rename = "idFfxVento")]
    id_ffx_wind: Option<i32>,
    #[serde(rename = "iUv")]
    i_uv: Option<String>,
    #[serde(rename = "idTipoTempo")]
    weather_type_id: Option<i32>,
    #[serde(rename = "globalIdLocal")]
    global_id_local: Option<i32>,
    #[serde(rename = "probabilidadePrecipita")]
    rain_probability: Option<String>,
    #[serde(rename = "idPeriodo")]
    id_periodo: Option<i32>,
    #[serde(rename = "dataPrev")]
    prevision_date: Option<String>,
    #[serde(rename = "ddVento")]
    dd_wind: Option<String>,
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
