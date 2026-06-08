use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaturationVaporPressureError {
    InvalidTemperature,
    TemperatureOutOfRange,
}

/// Computes saturation vapor pressure over liquid water.
///
/// Saturation vapor pressure is the maximum water vapor pressure air can hold
/// at a given temperature before condensation begins.
///
/// This uses the Bolton 1980 / Magnus-style approximation, found in NWS documentation:
///
/// `e_s = 6.112 * exp((17.67 * T) / (T + 243.5))`
///
/// where:
///
/// * `T` is temperature in degrees Celsius
/// * `e_s` is saturation vapor pressure in hectopascals
///
/// # Type Parameters
///
/// `T` can be any numeric type that implements [`ToPrimitive`] and can be
/// converted to `f64`.
///
/// # Arguments
///
/// * `temperature_c` - Air temperature in degrees Celsius.
///
/// # Returns
///
/// Returns `Ok(saturation_vapor_pressure)` in hectopascals.
///
/// Returns an error if the temperature cannot be converted to `f64`, is not
/// finite, or is outside the commonly quoted accuracy range for this formula.
pub fn saturation_vapor_pressure<T>(temperature_c: T) -> Result<f64, SaturationVaporPressureError>
where
    T: ToPrimitive,
{
    let temperature_c = temperature_c
        .to_f64()
        .ok_or(SaturationVaporPressureError::InvalidTemperature)?;

    if !temperature_c.is_finite() {
        return Err(SaturationVaporPressureError::InvalidTemperature);
    }

    // Bolton's formula is quoted as accurate to about 0.3%
    // for -35°C < T < 35°C, give some room to 45deg.
    if !(-35.0..=45.0).contains(&temperature_c) {
        return Err(SaturationVaporPressureError::TemperatureOutOfRange);
    }

    Ok(6.112 * ((17.67 * temperature_c) / (temperature_c + 243.5)).exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saturation_vapor_pressure_cases() {
        let cases = [
            // temperature C, expected saturation vapor pressure hPa
            (-20.0, 1.257),
            (-10.0, 2.867),
            (0.0, 6.112),
            (5.0, 8.721),
            (10.0, 12.271),
            (20.0, 23.369),
            (25.0, 31.674),
            (30.0, 42.455),
            (35.0, 56.311),
        ];

        for (temperature_c, expected) in cases {
            let actual = saturation_vapor_pressure(temperature_c)
                .expect("saturation_vapor_pressure should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "saturation_vapor_pressure({temperature_c}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_saturation_vapor_pressure_rejects_nan() {
        let actual = saturation_vapor_pressure(f64::NAN);

        assert_eq!(
            actual,
            Err(SaturationVaporPressureError::InvalidTemperature)
        );
    }

    #[test]
    fn test_saturation_vapor_pressure_rejects_out_of_range_temperature() {
        let actual = saturation_vapor_pressure(46.0);

        assert_eq!(
            actual,
            Err(SaturationVaporPressureError::TemperatureOutOfRange)
        );
    }
}
