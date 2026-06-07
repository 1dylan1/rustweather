use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DewPointError {
    InvalidVaporPressure,
    VaporPressureOutOfRange,
}

/// Computes dew point from vapor pressure.
///
/// Dew point is the temperature air must cool to for water vapor to begin
/// condensing. This function calculates dew point from actual vapor pressure.
///
/// # Type Parameters
///
/// `T` can be any numeric type that implements [`ToPrimitive`] and can be
/// converted to `f64`.
///
/// # Arguments
///
/// * `vapor_pressure_hpa` - Actual vapor pressure in hectopascals (millibars).
///
/// # Returns
///
/// Returns `Ok(dew_point_c)` in degrees Celsius.
///
/// Returns an error if vapor pressure cannot be converted to `f64`, is not
/// finite, or is less than or equal to zero.
pub fn dew_point_from_vapor_pressure<T>(vapor_pressure_hpa: T) -> Result<f64, DewPointError>
where
    T: ToPrimitive,
{
    let vapor_pressure_hpa = vapor_pressure_hpa
        .to_f64()
        .ok_or(DewPointError::InvalidVaporPressure)?;

    if !vapor_pressure_hpa.is_finite() {
        return Err(DewPointError::InvalidVaporPressure);
    }

    if vapor_pressure_hpa <= 0.0 {
        return Err(DewPointError::VaporPressureOutOfRange);
    }

    let gamma = (vapor_pressure_hpa / 6.112).ln();

    Ok((243.5 * gamma) / (17.67 - gamma))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dew_point_from_vapor_pressure() {
        let cases = vec![
            // vapor pressure hpa, expected dew point
            (6.112, 0.0),
            (15.112, 13.148),
            (49.201, 32.587),
        ];
        for (vapor_pressure, expected) in cases {
            let actual =
                dew_point_from_vapor_pressure(vapor_pressure).expect("dew point should return Ok");
            assert!(
                (actual - expected).abs() < 0.001,
                "dew_point_from_vapor_pressure({vapor_pressure}) = {actual}, expected {expected}."
            );
        }
    }
}
