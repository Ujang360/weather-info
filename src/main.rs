pub const TIMEOUT_GET: u64 = 1;

mod city_info;
mod weather_info;

use city_info::get_current_city;
use weather_info::get_current_weather;

fn main() {
    let current_city = get_current_city().unwrap();
    println!("{}", get_current_weather(&current_city).unwrap());
}
