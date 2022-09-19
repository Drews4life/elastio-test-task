use crate::weather::providers::{open_weather::OpenWeather, weather_api::WeatherApi};
use crate::weather::weather_provider::WeatherProvider;

pub struct Producer;

impl Producer {
    pub fn create(provider: &str) -> Box<dyn WeatherProvider> {
        match provider {
            "open_weather" => Box::new(OpenWeather::new()),
            "weather_api" => Box::new(WeatherApi::new()),
            _ => Box::new(WeatherApi::new()),
        }
    }
}
