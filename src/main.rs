mod cli;
mod coordinate;

use crate::cli::Args;
use crate::coordinate::Coordinates;

use clap::Parser;

use serde::{Deserialize, Serialize};

const WEATHER_SERVICE_API: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, Serialize, Deserialize)]
struct Current {
    interval: u32,
    temperature_2m: f64,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentWeather {
    current: Current,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let coordinates = Coordinates::new(args.latitude, args.longitude);
    let _weather = get_weather(coordinates)?;
    println!("{:#?}", _weather);
    Ok(())
}

#[tokio::main]
async fn get_weather(
    coordinates: Coordinates,
) -> Result<CurrentWeather, Box<dyn std::error::Error>> {
    let url = format!(
        "{0}?latitude={1}&longitude={2}&current=temperature_2m",
        WEATHER_SERVICE_API, coordinates.latitude, coordinates.longitude
    );
    let cw: CurrentWeather = reqwest::get(url).await?.json::<CurrentWeather>().await?;
    Ok(cw)
}
