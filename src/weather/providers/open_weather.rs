use crate::utils::extract_value_json;
use crate::weather::weather_provider::WeatherProvider;
use serde_json::Value;

pub struct OpenWeather;

struct LocationProvider;

struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl OpenWeather {
    pub fn new() -> Self {
        Self {}
    }
}

impl WeatherProvider for OpenWeather {
    fn get(&self, address: &str, _: i64) -> String {
        let location = LocationProvider::get_location_from_address(address)
            .expect("Was not able to get location coordinates :(");

        let url = format!(
            "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}",
            location.latitude, location.longitude, "32efec18adb00c64f4e9dab8e675429e",
        );

        let result = reqwest::blocking::get(url)
            .expect("Was not able to fetch forecast :(")
            .json::<Value>()
            .unwrap();

        let info = extract_value_json(&result, "list.0.weather.0.description")
            .expect("Unable to parse response");

        info.as_str().unwrap().to_owned()
    }
}

impl LocationProvider {
    pub fn get_location_from_address(address: &str) -> Result<Location, ()> {
        let query_params = format!(
            "?apikey=SJRPFcJgS8pN7X14v4vHWReJnmztxTGQ&q={}&language=en-us",
            address
        );
        let request_url = format!(
            "https://dataservice.accuweather.com/locations/v1/cities/search{}",
            query_params
        );
        let result = reqwest::blocking::get(request_url)
            .unwrap()
            .json::<Value>()
            .unwrap();

        let latitude = extract_value_json(&result, "0.GeoPosition.Latitude");
        let longitude = extract_value_json(&result, "0.GeoPosition.Longitude");

        if latitude.is_none() || longitude.is_none() {
            return Err(());
        }

        Ok(Location {
            longitude: longitude.unwrap().as_f64().unwrap(),
            latitude: latitude.unwrap().as_f64().unwrap(),
        })
    }
}
