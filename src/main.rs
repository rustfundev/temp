mod cli;

use crate::cli::Args;

use clap::Parser;

const WEATHER_SERVICE_API: &str = "https://api.open-meteo.com/v1/forecast";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let resp = reqwest::get(format!(
        "{0}?latitude={1}&longitude={2}&current=temperature_2m",
        WEATHER_SERVICE_API, args.latitude, args.longitude
    ))
    .await?
    .json::<serde_json::Value>()
    .await?;
    println!("{:#?}", resp);
    Ok(())
}
