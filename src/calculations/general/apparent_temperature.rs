use num_traits::ToPrimitive;

use super::heat_index::{HeatIndexError, heat_index};
use super::wind_chill::{WindChillError, wind_chill};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ApparentTemperatureError {
    InvalidTemperature,
    InvalidRelativeHumidity,
    InvalidWindSpeed,
    RelativeHumidityOutOfRange,
    HeatIndexError(HeatIndexError),
    WindChillError(WindChillError),
}

/// Computes the NWS-style apparent temperature.
///
/// Apparent temperature is the temperature the air feels like to the human body.
/// Depending on conditions, it may be based on wind chill, heat index, or the
/// actual air temperature.
///
/// NWS-style behavior:
///
/// * At or below `50°F`, apparent temperature is based on wind chill.
/// * Above `80°F`, apparent temperature is based on heat index.
/// * From `51°F` through `80°F`, apparent temperature is the actual air temperature.
///
/// # Type Parameters
///
/// `T`, `U`, and `V` can be any numeric types that implement [`ToPrimitive`] and
/// can be converted to `f64`.
///
/// # Arguments
///
/// * `temperature_f` - Air temperature in degrees Fahrenheit.
/// * `relative_humidity` - Relative humidity in percent, from `0.0` to `100.0`.
/// * `wind_speed_mph` - Wind speed in miles per hour.
///
/// # Returns
///
/// Returns `Ok(apparent_temperature)` in degrees Fahrenheit.
///
/// Returns an error if an input cannot be converted to `f64`, if relative
/// humidity is outside `0..=100`, or if the underlying heat index or wind chill
/// calculation fails.
///
/// # Notes
///
/// Relative humidity should be passed as a percentage.
///
/// Correct:
///
/// ```text
/// 65.0
/// ```
///
/// Not:
///
/// ```text
/// 0.65
/// ```
pub fn apparent_temperature<T, U, V>(
    temperature_f: T,
    relative_humidity: U,
    wind_speed_mph: V,
) -> Result<f64, ApparentTemperatureError>
where
    T: ToPrimitive,
    U: ToPrimitive,
    V: ToPrimitive,
{
    let temperature_f = temperature_f
        .to_f64()
        .ok_or(ApparentTemperatureError::InvalidTemperature)?;

    let relative_humidity = relative_humidity
        .to_f64()
        .ok_or(ApparentTemperatureError::InvalidRelativeHumidity)?;

    let wind_speed_mph = wind_speed_mph
        .to_f64()
        .ok_or(ApparentTemperatureError::InvalidWindSpeed)?;

    if !temperature_f.is_finite() {
        return Err(ApparentTemperatureError::InvalidTemperature);
    }

    if !relative_humidity.is_finite() {
        return Err(ApparentTemperatureError::InvalidRelativeHumidity);
    }

    if !wind_speed_mph.is_finite() {
        return Err(ApparentTemperatureError::InvalidWindSpeed);
    }

    if !(0.0..=100.0).contains(&relative_humidity) {
        return Err(ApparentTemperatureError::RelativeHumidityOutOfRange);
    }

    if temperature_f <= 50.0 {
        // NWS wind chill is only meaningful with wind speeds above 3 mph.
        // For calm or very light wind, use the actual air temperature.
        if wind_speed_mph <= 3.0 {
            return Ok(temperature_f);
        }

        return wind_chill(temperature_f, wind_speed_mph)
            .map_err(ApparentTemperatureError::WindChillError);
    }

    if temperature_f > 80.0 {
        return heat_index(temperature_f, relative_humidity)
            .map_err(ApparentTemperatureError::HeatIndexError);
    }

    Ok(temperature_f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apparent_temperature_uses_wind_chill() {
        let actual =
            apparent_temperature(30.0, 50.0, 10.0).expect("apparent_temperature should return Ok");

        assert!(
            (actual - 21.248).abs() < 0.001,
            "apparent_temperature returned {actual}"
        );
    }

    #[test]
    fn test_apparent_temperature_uses_air_temperature_for_light_wind_cold_case() {
        let actual =
            apparent_temperature(30.0, 50.0, 2.0).expect("apparent_temperature should return Ok");

        assert!((actual - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_apparent_temperature_uses_air_temperature_between_51_and_80() {
        let actual =
            apparent_temperature(70.0, 80.0, 10.0).expect("apparent_temperature should return Ok");

        assert!((actual - 70.0).abs() < 0.001);
    }

    #[test]
    fn test_apparent_temperature_uses_heat_index() {
        let actual =
            apparent_temperature(90.0, 70.0, 5.0).expect("apparent_temperature should return Ok");

        assert!(
            (actual - 105.922).abs() < 0.001,
            "apparent_temperature returned {actual}"
        );
    }

    #[test]
    fn test_apparent_temperature_normal() {
        let actual =
            apparent_temperature(90.0, 60.0, 5.0).expect("apparent temperature should return Ok");

        assert!(
            (actual - 99.677).abs() < 0.001,
            "apparent_temperature returned {actual}"
        )
    }

    #[test]
    fn test_apparent_temperature_rejects_invalid_relative_humidity() {
        let actual = apparent_temperature(90.0, 101.0, 5.0);

        assert_eq!(
            actual,
            Err(ApparentTemperatureError::RelativeHumidityOutOfRange)
        );
    }
}
