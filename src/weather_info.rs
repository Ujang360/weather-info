use crate::TIMEOUT_GET;
use dirs::home_dir;
use reqwest::blocking::{Client, Request};
use reqwest::Method;
use serde_json::{from_str as json_from_str, Value as JsonValue};
use std::fmt::{Display, Formatter, Result as FormatterResult};
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::time::Duration;

const PATH_API_KEY: &str = "openweathermap.apikey";

pub struct WeatherInfo {
    pub city: String,
    pub weather: String,
    pub temp: f64,
    pub temp_feels_like: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub wind_degree: f64,
}

impl Default for WeatherInfo {
    fn default() -> Self {
        Self {
            city: "London".into(),
            weather: "Clear".into(),
            temp: 0.0,
            temp_feels_like: 0.0,
            pressure: 0.0,
            humidity: 0.0,
            wind_speed: 0.0,
            wind_degree: 0.0,
        }
    }
}

impl Display for WeatherInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        write!(
            f,
            "
City           : {}
Weather        : {}
Temperature    : {} °C
Feels Like     : {} °C
Pressure       : {} mBar
Humidity       : {} %
Wind Speed     : {} m/s
Wind Direction : {} degree",
            self.city,
            self.weather,
            self.temp,
            self.temp_feels_like,
            self.pressure,
            self.humidity,
            self.wind_speed,
            self.wind_degree
        )
    }
}

fn get_api_key() -> Option<String> {
    let mut path_api_key = home_dir().unwrap();
    path_api_key.push(PATH_API_KEY);

    if !path_api_key.exists() && !prompt_save_api_key() {
        return None;
    }

    let file_open_result = File::open(path_api_key);

    if file_open_result.is_err() {
        return None;
    }

    let mut api_key = String::new();

    match file_open_result.unwrap().read_to_string(&mut api_key) {
        Err(_) => None,
        Ok(_) => Some(api_key),
    }
}

fn prompt_save_api_key() -> bool {
    println!("Please input your OpenWeatherMap API-Key (get it from \"https://home.openweathermap.org/api_keys\"):");
    let mut user_input = String::new();
    match stdin().read_line(&mut user_input) {
        Err(_) => false,
        Ok(_) => {
            let mut path_api_key = home_dir().unwrap();
            path_api_key.push(PATH_API_KEY);
            let mut file_create_result = File::create(path_api_key).unwrap();
            file_create_result.write(user_input.as_bytes()).unwrap();
            true
        }
    }
}

pub fn get_current_weather(city: &str) -> Option<WeatherInfo> {
    let api_key = get_api_key();

    if api_key.is_none() {
        return None;
    }

    let current_weather_uri = format!(
        "https://api.openweathermap.org/data/2.5/weather?units=metric&q={}&appid={}",
        city,
        api_key.unwrap()
    );
    let mut request = Request::new(
        Method::GET,
        current_weather_uri
            .parse()
            .expect("Bad current_weather_uri"),
    );
    request
        .timeout_mut()
        .replace(Duration::from_secs(TIMEOUT_GET));
    let client = Client::new();
    let response = client.execute(request).unwrap();
    let response_json: JsonValue = json_from_str(&response.text().unwrap()).unwrap();
    let response_map = response_json.as_object().unwrap().to_owned();
    let mut result = WeatherInfo::default();
    result.city = response_map["name"].as_str().unwrap().to_owned();
    result.weather = response_map["weather"].as_array().unwrap()[0]
        .as_object()
        .unwrap()["main"]
        .as_str()
        .unwrap()
        .to_owned();
    let weather_stats = response_map["main"].as_object().unwrap();
    result.temp = weather_stats["temp"].as_f64().unwrap();
    result.temp_feels_like = weather_stats["feels_like"].as_f64().unwrap();
    result.pressure = weather_stats["pressure"].as_f64().unwrap();
    result.humidity = weather_stats["humidity"].as_f64().unwrap();
    let wind_stats = response_map["wind"].as_object().unwrap();
    result.wind_speed = wind_stats["speed"].as_f64().unwrap();
    result.wind_degree = wind_stats["deg"].as_f64().unwrap();

    Some(result)
}
