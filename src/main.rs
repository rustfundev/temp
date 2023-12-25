mod cli;
mod coordinate;

use crate::cli::Args;
use crate::coordinate::Coordinates;

use clap::Parser;

const WEATHER_SERVICE_API: &str = "https://api.open-meteo.com/v1/forecast";

type Weather = serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let coordinates = Coordinates::new(args.latitude, args.longitude);
    let _weather = get_weather(coordinates)?;
    println!("{:#?}", _weather);
    Ok(())
}

#[tokio::main]
async fn get_weather(coordinates: Coordinates) -> Result<Weather, Box<dyn std::error::Error>> {
    let url = format!(
        "{0}?latitude={1}&longitude={2}&current=temperature_2m",
        WEATHER_SERVICE_API, coordinates.latitude, coordinates.longitude
    );
    let resp: Weather = reqwest::get(url).await?.json::<Weather>().await?;
    Ok(resp)
}
