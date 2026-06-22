use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PotentialTemperatureError {
    InvalidPressure,
    InvalidTemperature,
    PressureOutOfRange,
}

/// Computes the potential temperature from pressure and temperature.
///
/// Uses the Poisson equation to calculate potential temperature, which is the
/// temperature a parcel of dry air would have if brought isentropically (without
/// heat exchange) to a standard reference pressure of 1000 hPa.
///
/// Formula:
///
/// `θ = T * (P₀ / P)^κ`
///
/// where:
///
/// * `P₀` = 1000 hPa (standard reference pressure)
/// * `κ` = 0.286 (Poisson exponent, Rd/Cp)
///
/// # Type Parameters
///
/// `T` and `U` can be any numeric types that implement [`ToPrimitive`] and
/// can be converted to `f64`.
///
/// # Arguments
///
/// * `pressure_hpa` - Atmospheric pressure in hectopascals (millibars).
/// * `temperature_k` - Air temperature in Kelvin.
///
/// # Returns
///
/// Returns `Ok(potential_temperature)` in Kelvin.
///
/// Returns an error if an input cannot be converted to `f64`, if pressure
/// is not finite, or if pressure is less than or equal to zero.
pub fn potential_temperature<T, U>(
    pressure_hpa: T,
    temperature_k: U,
) -> Result<f64, PotentialTemperatureError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let pressure_hpa = pressure_hpa
        .to_f64()
        .ok_or(PotentialTemperatureError::InvalidPressure)?;

    let temperature_k = temperature_k
        .to_f64()
        .ok_or(PotentialTemperatureError::InvalidTemperature)?;

    if !pressure_hpa.is_finite() {
        return Err(PotentialTemperatureError::InvalidPressure);
    }

    if !temperature_k.is_finite() {
        return Err(PotentialTemperatureError::InvalidTemperature);
    }

    if pressure_hpa <= 0.0 {
        return Err(PotentialTemperatureError::PressureOutOfRange);
    }

    let kappa = 0.286;
    let p0 = 1000.0;

    Ok(temperature_k * (p0 / pressure_hpa).powf(kappa))
}

/// Computes the temperature from a given potential temperature and pressure.
///
/// This is the inverse of `potential_temperature`. Given a potential
/// temperature and the pressure at a given level, this returns the
/// actual air temperature at that level.
///
/// Formula:
///
/// `T = θ * (P / P₀)^κ`
///
/// where:
///
/// * `P₀` = 1000 hPa (standard reference pressure)
/// * `κ` = 0.286 (Poisson exponent, Rd/Cp)
///
/// # Type Parameters
///
/// `T` and `U` can be any numeric types that implement [`ToPrimitive`] and
/// can be converted to `f64`.
///
/// # Arguments
///
/// * `pressure_hpa` - Atmospheric pressure in hectopascals (millibars).
/// * `potential_temperature_k` - Potential temperature in Kelvin.
///
/// # Returns
///
/// Returns `Ok(temperature)` in Kelvin.
///
/// Returns an error if an input cannot be converted to `f64`, if pressure
/// is not finite, or if pressure is less than or equal to zero.
pub fn temperature_from_potential_temperature<T, U>(
    pressure_hpa: T,
    potential_temperature_k: U,
) -> Result<f64, PotentialTemperatureError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let pressure_hpa = pressure_hpa
        .to_f64()
        .ok_or(PotentialTemperatureError::InvalidPressure)?;

    let potential_temperature_k = potential_temperature_k
        .to_f64()
        .ok_or(PotentialTemperatureError::InvalidTemperature)?;

    if !pressure_hpa.is_finite() {
        return Err(PotentialTemperatureError::InvalidPressure);
    }

    if !potential_temperature_k.is_finite() {
        return Err(PotentialTemperatureError::InvalidTemperature);
    }

    if pressure_hpa <= 0.0 {
        return Err(PotentialTemperatureError::PressureOutOfRange);
    }

    let kappa = 0.286;
    let p0 = 1000.0;

    Ok(potential_temperature_k * (pressure_hpa / p0).powf(kappa))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potential_temperature_cases() {
        let cases = vec![
            // pressure_hpa, temperature_k, expected_theta_k (computed with kappa=0.286)
            (800.0, 273.0, 290.991),
            (1000.0, 300.0, 300.0),
            (500.0, 250.0, 304.814),
            (1013.25, 288.15, 287.067),
            (700.0, 240.0, 265.774),
        ];

        for (pressure_hpa, temperature_k, expected) in cases {
            let actual = potential_temperature(pressure_hpa, temperature_k)
                .expect("potential_temperature should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "potential_temperature({pressure_hpa}, {temperature_k}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_temperature_from_potential_temperature_cases() {
        let cases = vec![
            // pressure_hpa, potential_temperature_k, expected_temperature_k
            (800.0, 290.991, 273.0),
            (1000.0, 300.0, 300.0),
            (500.0, 304.814, 250.0),
            (1013.25, 287.067, 288.15),
            (700.0, 265.774, 240.0),
        ];

        for (pressure_hpa, theta_k, expected) in cases {
            let actual = temperature_from_potential_temperature(pressure_hpa, theta_k)
                .expect("temperature_from_potential_temperature should return Ok");

            assert!(
                (actual - expected).abs() < 0.01,
                "temperature_from_potential_temperature({pressure_hpa}, {theta_k}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_roundtrip() {
        let pressure = 750.0;
        let temperature = 280.0;
        let theta = potential_temperature(pressure, temperature).unwrap();
        let t_back = temperature_from_potential_temperature(pressure, theta).unwrap();

        assert!(
            (t_back - temperature).abs() < 0.001,
            "roundtrip failed: started with {temperature}, got {t_back}"
        );
    }

    #[test]
    fn test_at_reference_pressure() {
        let actual = potential_temperature(1000.0, 300.0).unwrap();
        assert!(
            (actual - 300.0).abs() < 0.001,
            "potential_temperature at P0 should equal temperature, got {actual}"
        );
    }

    #[test]
    fn test_pressure_out_of_range() {
        let actual = potential_temperature(0.0, 300.0);
        assert_eq!(actual, Err(PotentialTemperatureError::PressureOutOfRange));

        let actual = potential_temperature(-100.0, 300.0);
        assert_eq!(actual, Err(PotentialTemperatureError::PressureOutOfRange));
    }

    #[test]
    fn test_rejects_nan_pressure() {
        let actual = potential_temperature(f64::NAN, 300.0);
        assert_eq!(actual, Err(PotentialTemperatureError::InvalidPressure));
    }

    #[test]
    fn test_rejects_nan_temperature() {
        let actual = potential_temperature(800.0, f64::NAN);
        assert_eq!(actual, Err(PotentialTemperatureError::InvalidTemperature));
    }

    #[test]
    fn test_rejects_infinite_pressure() {
        let actual = potential_temperature(f64::INFINITY, 300.0);
        assert_eq!(actual, Err(PotentialTemperatureError::InvalidPressure));
    }
}
