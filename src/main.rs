use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::get;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Weather {
}

fn main() -> Result<(), Box<dyn Error>> {
    let resp = match get("https://api.ipma.pt/public-data/forecast/aggregate/1030300.json") {
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
        
        let data: Value = match serde_json::from_str(&body) {
            Ok(data) => data,
            Err(err) => {
                println!("Error parsing JSON: {}", err);
                return Ok(());
            }
        };

        println!("{:?}", data);
    } else {
        println!("Error: {}", resp.status());
    }

    Ok(())
}
