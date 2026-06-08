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

You'll notice that some functions require varying units for the parameters. To help alleviate some of the work,
`helpers` has a section that will be continually adding varying conversion functions, such as temperature between
both degrees and kelvin.

## Installation

After the crate is published to crates.io:

```toml
[dependencies]
rustweather = "0.1.0"
```

For local development:

```toml
[dependencies]
rustweather = { path = "../rustweather" }
```

## Usage

```rust
use rustweather::calculations::general::{
    wind_speed,
    wind_direction,
    wind_components,
    wind_chill,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let speed = wind_speed(3.0, 4.0)?;
    println!("Wind speed: {speed} m/s");

    let direction = wind_direction(4.0, 0.0)?;
    println!("Wind direction: {direction}°");

    let (u, v) = wind_components(10.0, 270.0)?;
    println!("u component: {u}");
    println!("v component: {v}");

    let chill = wind_chill(5.0, 30.0)?;
    println!("Wind chill: {chill}°F");

    Ok(())
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
| `dew_point_from_vapor_pressure(vapor_pressure)` | hectopascals (millibars) | degrees celsius |
| `dew_point_from_temp_and_rh(temp_c, rh)` | degrees Celsius, percentage | degrees celsius |
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
