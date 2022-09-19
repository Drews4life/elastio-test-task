use crate::utils::extract_value_json;
use crate::weather::weather_provider::WeatherProvider;
use serde_json::Value;

pub struct WeatherApi;

impl WeatherApi {
    pub fn new() -> Self {
        Self {}
    }
}

impl WeatherProvider for WeatherApi {
    fn get(&self, address: &str, _: i64) -> String {
        let url = format!("https://api.weatherapi.com/v1/current.json?key=0f8d1b58e1e942a3bb5133332221909&q={}&aqi=no", address);
        let result = reqwest::blocking::get(url)
            .expect("Could not get forecast :(")
            .json::<Value>()
            .unwrap();

        let info = extract_value_json(&result, "current.condition.text").unwrap();

        info.as_str().unwrap().to_owned()
    }
}
