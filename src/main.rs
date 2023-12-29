mod cli;
mod coordinate;
mod request;

use crate::cli::Args;
use crate::coordinate::Coordinates;
use crate::request::Request;

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

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    current: Current,
}

impl Weather {
    fn get() {}
}

impl Request for Weather {
    #[tokio::main]
    async fn get(&self, coordinates: &Coordinates) -> String {
        let url = format!(
            "{0}?latitude={1}&longitude={2}&current=temperature_2m",
            WEATHER_SERVICE_API, coordinates.latitude, coordinates.longitude
        );
        let cw: String = reqwest::get(url)
            .await
            .expect("Error something")
            .json::<String>()
            .await
            .expect("Error something else");
        cw
    }
}

#[tokio::main]
async fn get_weather(
    coordinates: Coordinates,
) -> Result<CurrentWeather, Box<dyn std::error::Error>> {
    if coordinates.latitude == 0.0 || coordinates.longitude == 0.0 {
        panic!("This should never happen");
    }

    let url = format!(
        "{0}?latitude={1}&longitude={2}&current=temperature_2m",
        WEATHER_SERVICE_API, coordinates.latitude, coordinates.longitude
    );
    let cw: CurrentWeather = reqwest::get(url).await?.json::<CurrentWeather>().await?;
    Ok(cw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_weather() -> Result<(), Box<dyn std::error::Error>> {
        // TODO: add mock
        let coordinate: Coordinates = Coordinates::new(10.0, 10.0);
        let _weather = get_weather(coordinate)?;
        assert_eq!(_weather.current.interval, 900);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_get_weather_panic() {
        let coordinate: Coordinates = Coordinates::new(0.0, 0.0);
        _ = get_weather(coordinate);
    }
}
