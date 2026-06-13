use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VirtualTemperatureError {
    InvalidTemperature,
    InvalidDewPoint,
    InvalidPressure,
}

/// Computes the NWS-style virtual temperature.
///
/// Virtual temperature is the temperature that dry air would need to be to
/// weight the exact same as moist (eg humid) air.
///
/// # Type Parameters
///
/// `T`, `U`, and `V` can be any numeric types that implement [`ToPrimitive`] and
/// can be converted to `f64`.
///
/// # Arguments
///
/// * `temperature_c` - Air temperature in degrees Celsius.
/// * `dewpoint_c` - Dewpoint temperature in degrees Celsius.
/// * `pressure_hpa` - Pressure in hectopascals (millibars).
///
/// # Returns
///
/// Returns `Ok(virtual_temperature)` in Kelvin.
///
/// Returns an error if an input cannot be converted to `f64`
pub fn virtual_temperature<T, U, V>(
    temperature_c: T,
    dewpoint_c: U,
    pressure_hpa: V,
) -> Result<f64, VirtualTemperatureError>
where
    T: ToPrimitive,
    U: ToPrimitive,
    V: ToPrimitive,
{
    let temperature_c = temperature_c
        .to_f64()
        .ok_or(VirtualTemperatureError::InvalidTemperature)?;

    let dewpoint_c = dewpoint_c
        .to_f64()
        .ok_or(VirtualTemperatureError::InvalidDewPoint)?;

    let pressure_hpa = pressure_hpa
        .to_f64()
        .ok_or(VirtualTemperatureError::InvalidPressure)?;

    let numerator = temperature_c + 273.15;
    let denominator =
        0.379 * ((6.11 * 10.0_f64.powf((7.5 * dewpoint_c) / (237.3 + dewpoint_c))) / pressure_hpa);

    return Ok(numerator / (1.0 - denominator));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_temperature() {
        let cases = vec![
            // temp_c, dewpoint_c, pressure_hpa, expected_K
            (35.0, 25.0, 1013.0, 311.846),
            (0.0, 0.0, 1025.0, 273.768),
        ];
        for (temp_c, dewpoint_c, pressure_hpa, expected) in cases {
            let actual = virtual_temperature(temp_c, dewpoint_c, pressure_hpa)
                .expect("virtual temperature should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "virtual_temperature({temp_c}, {dewpoint_c}, {pressure_hpa}) = {actual}, expected {expected}"
            );
        }
    }
}
