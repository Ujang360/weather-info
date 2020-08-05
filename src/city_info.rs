use crate::TIMEOUT_GET;
use reqwest::blocking::{Client, Request};
use reqwest::Method;
use serde_json::{from_str as json_from_str, Value as JsonValue};
use std::boxed::Box;
use std::error::Error;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::time::Duration;

const URI_GET_CITY: &str = "http://ip-api.com/json/?fields=city";

pub fn get_current_city() -> Result<String, Box<dyn Error>> {
    let mut request = Request::new(Method::GET, URI_GET_CITY.parse().expect("Bad URI_GET_CITY"));
    request
        .timeout_mut()
        .replace(Duration::from_secs(TIMEOUT_GET));
    let client = Client::new();
    let response = client.execute(request)?;
    let string_result = response.text()?;
    let json_result: Result<JsonValue, _> = json_from_str(&string_result);

    if json_result.is_err() {
        return Err(Box::new(IOError::new(
            IOErrorKind::InvalidData,
            "Invalid response from URI_GET_CITY (Body is not JSON)!",
        )));
    }

    let json_result = json_result.unwrap();
    let json_result = json_result.as_object();

    if json_result.is_none() {
        return Err(Box::new(IOError::new(
            IOErrorKind::InvalidData,
            "Invalid response from URI_GET_CITY (Not JSON Object)!",
        )));
    }

    let json_result = json_result.unwrap().to_owned();

    if !json_result.contains_key("city") {
        return Err(Box::new(IOError::new(
            IOErrorKind::InvalidData,
            "Invalid response from URI_GET_CITY (No city found)!",
        )));
    }

    Ok(json_result["city"].as_str().unwrap().to_owned())
}
