pub trait WeatherProvider {
    fn get(&self, address: &str, timestamp: i64) -> String;
}
