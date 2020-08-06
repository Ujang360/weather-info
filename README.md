# Weather Info

An [OpenWeather](https://openweathermap.org/) terminal utility to get current location's weather condition. This is pretty much a hobby project.

## Installation

To install this from Rust [community's crate registry](https://crates.io/), one must [install Rust](https://www.rust-lang.org/tools/install). Then do this in the terminal:

```bash
cargo install weather-info
```

## Usage

```bash
> weather-info

City           : Bandung
Weather        : Clouds
Temperature    : 27.69 °C
Feels Like     : 30.02 °C
Pressure       : 1011 mBar
Humidity       : 58 %
Wind Speed     : 1.08 m/s
Wind Direction : 346 degree

```

## Usage Note

You will be prompted to type OpenWeather API-Key, it will save the key in your home folder. One can get the key from [this link](https://home.openweathermap.org/api_keys) for free.

## License

`weather-info` is distributed under the terms the MIT license.

See [LICENSE](LICENSE) for details.
