use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeatIndexError {
    InvalidTemperature,
    InvalidRelativeHumidity,
    RelativeHumidityOutOfRange,
}

/// Computes the NWS heat index from air temperature and relative humidity.
///
/// This uses the Rothfusz regression equation used by the National Weather
/// Service, with the low-humidity and high-humidity adjustments.
///
/// For lower apparent temperatures, the simpler Steadman-style formula is used
/// first. If that preliminary heat index is below `80°F`, that value is returned
/// instead of applying the full Rothfusz regression.
///
/// # Type Parameters
///
/// `T` and `U` can be any numeric types that implement [`ToPrimitive`] and can
/// be converted to `f64`.
///
/// # Arguments
///
/// * `temperature_f` - Air temperature in degrees Fahrenheit.
/// * `relative_humidity` - Relative humidity in percent, from `0.0` to `100.0`.
///
/// # Returns
///
/// Returns `Ok(heat_index)` in degrees Fahrenheit.
///
/// Returns an error if either input cannot be converted to `f64`, if either
/// input is not finite, or if relative humidity is outside `0..=100`.
///
/// # Notes
///
/// This function expects relative humidity as a percentage.
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
pub fn heat_index<T, U>(temperature_f: T, relative_humidity: U) -> Result<f64, HeatIndexError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let temperature_f = temperature_f
        .to_f64()
        .ok_or(HeatIndexError::InvalidTemperature)?;

    let relative_humidity = relative_humidity
        .to_f64()
        .ok_or(HeatIndexError::InvalidRelativeHumidity)?;

    if !temperature_f.is_finite() {
        return Err(HeatIndexError::InvalidTemperature);
    }

    if !relative_humidity.is_finite() {
        return Err(HeatIndexError::InvalidRelativeHumidity);
    }

    if !(0.0..=100.0).contains(&relative_humidity) {
        return Err(HeatIndexError::RelativeHumidityOutOfRange);
    }

    let simple_heat_index =
        0.5 * (temperature_f + 61.0 + ((temperature_f - 68.0) * 1.2) + (relative_humidity * 0.094));

    if simple_heat_index < 80.0 {
        return Ok(simple_heat_index);
    }

    let mut heat_index = -42.379 + 2.049_015_23 * temperature_f + 10.143_331_27 * relative_humidity
        - 0.224_755_41 * temperature_f * relative_humidity
        - 0.006_837_83 * temperature_f * temperature_f
        - 0.054_817_17 * relative_humidity * relative_humidity
        + 0.001_228_74 * temperature_f * temperature_f * relative_humidity
        + 0.000_852_82 * temperature_f * relative_humidity * relative_humidity
        - 0.000_001_99 * temperature_f * temperature_f * relative_humidity * relative_humidity;

    if relative_humidity < 13.0 && (80.0..=112.0).contains(&temperature_f) {
        let adjustment = ((13.0 - relative_humidity) / 4.0)
            * ((17.0 - (temperature_f - 95.0).abs()) / 17.0).sqrt();

        heat_index -= adjustment;
    } else if relative_humidity > 85.0 && (80.0..=87.0).contains(&temperature_f) {
        let adjustment = ((relative_humidity - 85.0) / 10.0) * ((87.0 - temperature_f) / 5.0);

        heat_index += adjustment;
    }

    return Ok(heat_index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_index_cases() {
        let cases = [
            // temp_f, rh_percent, expected_heat_index
            (79.0, 50.0, 78.950),
            (80.0, 40.0, 79.580),
            (90.0, 70.0, 105.922),
            (95.0, 50.0, 105.215),
            (100.0, 40.0, 109.255),
            (85.0, 90.0, 101.780),
            (90.0, 10.0, 85.278),
        ];

        for (temperature_f, relative_humidity, expected) in cases {
            let actual =
                heat_index(temperature_f, relative_humidity).expect("heat_index should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "heat_index({temperature_f}, {relative_humidity}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_heat_index_relative_humidity_too_low() {
        let actual = heat_index(90.0, -1.0);

        assert_eq!(actual, Err(HeatIndexError::RelativeHumidityOutOfRange));
    }

    #[test]
    fn test_heat_index_relative_humidity_too_high() {
        let actual = heat_index(90.0, 101.0);

        assert_eq!(actual, Err(HeatIndexError::RelativeHumidityOutOfRange));
    }
}
