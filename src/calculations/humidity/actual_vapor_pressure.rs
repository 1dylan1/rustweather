use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActualVaporPressureError {
    InvalidTemperature,
    TemperatureOutOfRange,
}

/// Computes actual vapor pressure from dewpoint temperature.
///
/// Actual vapor pressure is the current water vapor pressure in the air.
///
/// This uses the Magnus-style approximation shown in NWS documentation.
///
/// where:
///
/// * `Td` is dewpoint temperature in degrees Celsius
/// * `e` is actual vapor pressure in hectopascals
///
/// # Type Parameters
///
/// `T` can be any numeric type that implements [`ToPrimitive`] and can be
/// converted to `f64`.
///
/// # Arguments
///
/// * `dewpoint_c` - Dewpoint temperature in degrees Celsius.
///
/// # Returns
///
/// Returns `Ok(actual_vapor_pressure)` in hectopascals.
///
/// Returns an error if the dewpoint cannot be converted to `f64`, is not
/// finite, or is outside the commonly used range for this formula.
pub fn actual_vapor_pressure<T>(dewpoint_c: T) -> Result<f64, ActualVaporPressureError>
where
    T: ToPrimitive,
{
    let dewpoint_c = dewpoint_c
        .to_f64()
        .ok_or(ActualVaporPressureError::InvalidTemperature)?;

    if !dewpoint_c.is_finite() {
        return Err(ActualVaporPressureError::InvalidTemperature);
    }

    // Same practical range as our saturation vapor pressure function. See `saturated_vapor_pressure.rs`
    if !(-35.0..=45.0).contains(&dewpoint_c) {
        return Err(ActualVaporPressureError::TemperatureOutOfRange);
    }

    return Ok(6.11 * 10.0_f64.powf((7.5 * dewpoint_c) / (237.3 + dewpoint_c)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actual_vapor_pressure_cases() {
        let cases = [
            // dewpoint c, expected actual vapor pressure hpa
            (-20.0, 1.247),
            (-10.0, 2.858),
            (0.0, 6.110),
            (5.0, 8.726),
            (10.0, 12.283),
            (20.0, 23.389),
            (25.0, 31.686),
            (30.0, 42.442),
            (35.0, 56.241),
        ];

        for (dewpoint_c, expected) in cases {
            let actual =
                actual_vapor_pressure(dewpoint_c).expect("actual_vapor_pressure should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "actual_vapor_pressure({dewpoint_c}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_actual_vapor_pressure_rejects_nan() {
        let actual = actual_vapor_pressure(f64::NAN);

        assert_eq!(actual, Err(ActualVaporPressureError::InvalidTemperature));
    }

    #[test]
    fn test_actual_vapor_pressure_rejects_out_of_range_temperature() {
        let actual = actual_vapor_pressure(46.0);

        assert_eq!(actual, Err(ActualVaporPressureError::TemperatureOutOfRange));
    }
}
