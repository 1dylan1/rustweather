# rustweather

A lightweight Rust library for meteorological parameter calculations.

`rustweather` provides reusable calculation utilities for common atmospheric and weather-related parameters. It is designed to be small, dependency-light, and suitable for use in weather applications, data processing pipelines, command-line tools, and scientific software.

> [!IMPORTANT]
> The long-term goal of this crate is to provide a Rust-native suite of meteorological calculation utilities comparable in scope to MetPy’s calculation tools in the Python ecosystem.
> It is not intended to be a drop-in replacement for MetPy, as MetPy covers other features beyond just weather calculations.

> [!TIP]
> Auto generated docs for the most recent main commit are available at https://1dylan1.github.io/rustweather/rustweather/index.html

## Features

Currently supported calculations include:

- Wind speed from `u` and `v` wind components
- Meteorological wind direction from `u` and `v` wind components
- `u` and `v` wind components from wind speed and direction
- Wind chill temperature index
- Pressure to height
- Apparent temperature
- Heat index
- Geometric height to geopotential height
- Geometric height to geopotential
- Geometric height to atmospheric pressure
- Dew point from vapor pressure
- Dew point from temperature & relative humidity
- Wet bulb temperature
- Saturated vapor pressure
- Actual vapor pressure
- Actual mixing ratio
- Saturated mixing ratio
- Relative humidity from saturated/actual vapor pressure
- Relative humidity from temperature/dew point
- Wind Shear
- Virtual temperature
- Potential temperature
 
You'll notice that some functions require varying units for the parameters. To help alleviate some of the work,
`helpers` has a section that will be continually adding varying conversion functions, such as temperature between
both degrees and kelvin.

## Installation

```toml
cargo add rustweather
```

For local development:

```toml
[dependencies]
rustweather = { path = "../rustweather" }
```

## Usage

```rust
use rustweather::calculations::general::wind_speed;
use rustweather::calculations::helpers::celsius_to_fahrenheit;

fn main() {
    let speed = wind_speed(3.0, 4.0).unwrap();
    println!("Wind speed: {speed} m/s");

    let temp_f = celsius_to_fahrenheit(0.0).unwrap();
    println!("0C = {temp_f}F");
}
```

## Units

This crate does not perform automatic unit conversion. Inputs must be provided using the units expected by each function, which are provided in their function documentation, and the table below:

| Function | Input Units | Output Units |
|---|---|---|
| `wind_speed(u, v)` | meters per second | meters per second |
| `wind_direction(u, v)` | meters per second | degrees |
| `wind_components(speed, direction)` | any speed unit, degrees | same speed unit |
| `wind_chill(temp_f, speed_mph)` | degrees Fahrenheit, miles per hour | degrees Fahrenheit |
| `apparent_temperature(temp_f, rh, wind_speed_mph)` | degrees Fahrenheit, percentage, miles per hours | degrees Fahrenheit |
| `heat_index(temp_f, rh)` | degrees Fahrenheit, percentage | degrees Fahrenheit | 
| `pressure_to_height(pressure)` | hectopascals (millibars) | feet |
| `geometric_to_geopotential_height(height_m)` | meters | geopotential meters |
| `height_to_geopotential(height_m)` | meters | square meters per second squared, `m^2/s^2` |
| `height_to_pressure(height_m)` | meters | hectopascals (millibars) |
| `dew_point_from_vapor_pressure(vapor_pressure)` | hectopascals (millibars) | degrees Celsius |
| `dew_point_from_temp_and_rh(temp_c, rh)` | degrees Celsius, percentage | degrees Celsius |
| `wet_bulb_temperature(temp_c, rh, pressure)` | degrees Celsius, percentage, hectopascalas (millibars) | degrees Celsius |
| `saturated_vapor_pressure(temp_c)` | degrees Celsius | hectopascals (millibars) |
| `actual_vapor_pressure(dew_pt)` | degrees Celsius | hectopascals (millibars) |
| `actual_mixing_ratio(pressure, temp_c)` | hectopascals (millibars), degrees Celsius | grams per kilogram (g/kg) |
| `saturated_mixing_ratio(pressure, temp_c)` | hectopascals (millibars), degrees Celsius | grams per kilogram (g/kg) |
| `relative_humidity_from_mixing_ratio(actual_mix_ratio, saturated_mix_ratio)` | grams per kilogram, grams per kilogram | percentage (0-100) |
| `relative_humidity_from_temp_dewpoint(temp_c, dewpoint_c)` | degrees Celsius, degrees Celsius | percentage (0-100) |
| `wind_shear_from_components(lower_u, lower_v, upper_u, upper_v)` | wind component, wind component, wind component, wind component | wind shear magnitude (same units as input components) |
| `wind_shear_from_direction_speed(lower_direction_deg, lower_speed, upper_direction_deg, upper_speed)` | meteorological degrees, wind speed, meteorological degrees, wind speed | wind shear magnitude (same units as input wind speeds) |
| `bulk_shear(lower_u, lower_v, upper_u, upper_v)` | wind component, wind component, wind component, wind component | bulk wind shear magnitude (same units as input components) |
| `virtual_temperature(temp_c, dewpoint_c, pressure)` | degrees Celsius, degrees Celsius, millibars (hectopascals) | Kelvin |
| `potential_temperature(pressure, temp_k)` | millibars (hectopascals), Kelvin | Kelvin |
| `temperature_from_potential_temperature(pressure, potential_temp_k)` | millibars (hectopascals), Kelvin | Kelvin |

`wind_components` preserves the input speed unit. For example, if speed is provided in meters per second, the returned `u` and `v` components are also in meters per second.

## Error Handling

Functions return `Result` when input conversion or formula validation can fail.

Example:

```rust
let speed = wind_speed(3.0, 4.0)?;

let invalid = wind_chill(70.0, 10.0);
assert!(invalid.is_err());
```

## Testing

Run the full test suite:

```sh
cargo test
```

List all discovered tests:

```sh
cargo test -- --list
```

Run tests matching a specific name:

```sh
cargo test wind_speed
```
