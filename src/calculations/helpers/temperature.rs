use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureConversionError {
    InvalidTemperature,
}

/// Converts Celsius to Fahrenheit.
pub fn celsius_to_fahrenheit<T>(temperature: T) -> Result<f64, TemperatureConversionError>
where
    T: ToPrimitive,
{
    let temperature = temperature
        .to_f64()
        .ok_or(TemperatureConversionError::InvalidTemperature)?;

    Ok((temperature * 9.0 / 5.0) + 32.0)
}

/// Converts Fahrenheit to Celsius.
pub fn fahrenheit_to_celsius<T>(temperature: T) -> Result<f64, TemperatureConversionError>
where
    T: ToPrimitive,
{
    let temperature = temperature
        .to_f64()
        .ok_or(TemperatureConversionError::InvalidTemperature)?;

    Ok((temperature - 32.0) * 5.0 / 9.0)
}

/// Converts Celsius to Kelvin.
pub fn celsius_to_kelvin<T>(temperature: T) -> Result<f64, TemperatureConversionError>
where
    T: ToPrimitive,
{
    let temperature = temperature
        .to_f64()
        .ok_or(TemperatureConversionError::InvalidTemperature)?;

    Ok(temperature + 273.15)
}

/// Converts Kelvin to Celsius.
pub fn kelvin_to_celsius<T>(temperature: T) -> Result<f64, TemperatureConversionError>
where
    T: ToPrimitive,
{
    let temperature = temperature
        .to_f64()
        .ok_or(TemperatureConversionError::InvalidTemperature)?;

    Ok(temperature - 273.15)
}

/// Converts Fahrenheit to Kelvin.
pub fn fahrenheit_to_kelvin<T>(temperature: T) -> Result<f64, TemperatureConversionError>
where
    T: ToPrimitive,
{
    let temperature = temperature
        .to_f64()
        .ok_or(TemperatureConversionError::InvalidTemperature)?;

    Ok((temperature - 32.0) * 5.0 / 9.0 + 273.15)
}

/// Converts Kelvin to Fahrenheit.
pub fn kelvin_to_fahrenheit<T>(temperature: T) -> Result<f64, TemperatureConversionError>
where
    T: ToPrimitive,
{
    let temperature = temperature
        .to_f64()
        .ok_or(TemperatureConversionError::InvalidTemperature)?;

    Ok((temperature - 273.15) * 9.0 / 5.0 + 32.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_conversions() {
        assert!((celsius_to_fahrenheit(0.0).unwrap() - 32.0).abs() < 0.001);
        assert!((fahrenheit_to_celsius(32.0).unwrap() - 0.0).abs() < 0.001);

        assert!((celsius_to_kelvin(0.0).unwrap() - 273.15).abs() < 0.001);
        assert!((kelvin_to_celsius(273.15).unwrap() - 0.0).abs() < 0.001);

        assert!((fahrenheit_to_kelvin(32.0).unwrap() - 273.15).abs() < 0.001);
        assert!((kelvin_to_fahrenheit(273.15).unwrap() - 32.0).abs() < 0.001);
    }
}
