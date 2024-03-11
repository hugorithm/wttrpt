use serde::Deserialize;
use std::collections::HashMap;
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

#[derive(Debug, Deserialize)]
pub struct WeatherDescription {
    #[serde(rename = "PT")]
    pt: String,
    #[serde(rename = "EN")]
    en: String,
}

pub fn get_weather_types() -> Result<HashMap<String, WeatherDescription>, Box<dyn Error>> {
    let resp = match reqwest::blocking::get("https://www.ipma.pt/bin/file.data/weathertypes.json") {
        Ok(resp) => resp,
        Err(err) => {
            return Err(err.into());
        }
    };

    if resp.status().is_success() {
        let body = match resp.text() {
            Ok(body) => body,
            Err(err) => {
                return Err(err.into());
            }
        };

        let weather_types: HashMap<String, WeatherDescription> = match serde_json::from_str(&body) {
            Ok(entries) => entries,
            Err(err) => {
                return Err(err.into());
            }
        };

        Ok(weather_types)
    } else {
        Err(format!("Error: {}", resp.status()).into())
    }
}

pub fn get_weather() -> Result<Vec<Weather>, Box<dyn Error>> {
    let resp = match reqwest::blocking::get(
        "https://api.ipma.pt/public-data/forecast/aggregate/1030300.json",
    ) {
        Ok(resp) => resp,
        Err(err) => {
            return Err(err.into());
        }
    };

    if resp.status().is_success() {
        let body = match resp.text() {
            Ok(body) => body,
            Err(err) => {
                return Err(err.into());
            }
        };

        let weather_data: Vec<Weather> = match serde_json::from_str(&body) {
            Ok(entries) => entries,
            Err(err) => {
                return Err(err.into());
            }
        };

        Ok(weather_data)
    } else {
        Err(format!("Error: {}", resp.status()).into())
    }
}
