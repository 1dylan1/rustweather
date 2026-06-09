use num_traits::ToPrimitive;

use crate::calculations::humidity::saturated_vapor_pressure::{
    SaturationVaporPressureError, saturation_vapor_pressure,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WetBulbError {
    InvalidTemperature,
    InvalidRelativeHumidity,
    InvalidPressure,
    RelativeHumidityOutOfRange,
    PressureOutOfRange,
    SaturationVaporPressureError(SaturationVaporPressureError),
    DidNotConverge,
}

/// Computes wet-bulb temperature from air temperature, relative humidity,
/// and pressure.
///
/// Wet-bulb temperature is the temperature air would cool to through
/// evaporation at constant pressure. It is useful for humidity, heat stress,
/// and evaporative cooling calculations.
///
/// This follows the NWS-style iterative calculation using air
/// temperature, relative humidity, and pressure.
///
/// # Arguments
///
/// * `air_temperature_c` - Air temperature in degrees Celsius.
/// * `relative_humidity` - Relative humidity in percent, from `0.0` to `100.0`.
/// * `pressure_hpa` - pressure in hectopascals / millibars.
///
/// # Returns
///
/// Returns `Ok(wet_bulb_temperature)` in degrees Celsius.
pub fn wet_bulb<T, U, V>(
    air_temperature_c: T,
    relative_humidity: U,
    pressure_hpa: V,
) -> Result<f64, WetBulbError>
where
    T: ToPrimitive,
    U: ToPrimitive,
    V: ToPrimitive,
{
    let air_temperature_c = air_temperature_c
        .to_f64()
        .ok_or(WetBulbError::InvalidTemperature)?;

    let relative_humidity = relative_humidity
        .to_f64()
        .ok_or(WetBulbError::InvalidRelativeHumidity)?;

    let pressure_hpa = pressure_hpa.to_f64().ok_or(WetBulbError::InvalidPressure)?;

    if !air_temperature_c.is_finite() {
        return Err(WetBulbError::InvalidTemperature);
    }

    if !relative_humidity.is_finite() {
        return Err(WetBulbError::InvalidRelativeHumidity);
    }

    if !pressure_hpa.is_finite() {
        return Err(WetBulbError::InvalidPressure);
    }

    if !(0.0..=100.0).contains(&relative_humidity) {
        return Err(WetBulbError::RelativeHumidityOutOfRange);
    }

    if pressure_hpa <= 0.0 {
        return Err(WetBulbError::PressureOutOfRange);
    }

    let saturation_vapor_pressure_hpa = saturation_vapor_pressure(air_temperature_c)
        .map_err(WetBulbError::SaturationVaporPressureError)?;

    let actual_vapor_pressure_hpa = saturation_vapor_pressure_hpa * (relative_humidity / 100.0);

    let mut wet_bulb_guess_c: f64 = 0.0;
    let mut increment = 10.0;
    let mut previous_sign = 1.0;

    for _ in 0..10_000 {
        let saturation_at_guess_hpa =
            6.112 * ((17.67 * wet_bulb_guess_c) / (wet_bulb_guess_c + 243.5)).exp();

        let guessed_vapor_pressure_hpa = saturation_at_guess_hpa
            - pressure_hpa
                * (air_temperature_c - wet_bulb_guess_c)
                * 0.000_66
                * (1.0 + 0.001_15 * wet_bulb_guess_c);

        let difference = actual_vapor_pressure_hpa - guessed_vapor_pressure_hpa;

        if difference.abs() <= 0.005 {
            return Ok(wet_bulb_guess_c);
        }

        let current_sign = if difference < 0.0 { -1.0 } else { 1.0 };

        if current_sign != previous_sign {
            previous_sign = current_sign;
            increment /= 10.0;
        }

        wet_bulb_guess_c += increment * previous_sign;
    }

    return Err(WetBulbError::DidNotConverge);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wet_bulb_cases() {
        let cases = [
            // air temp C, RH , pressure hPa, expected wet bulb C
            (30.0, 50.0, 1013.25, 22.122),
            (30.0, 70.0, 1013.25, 25.570),
            (25.0, 50.0, 1013.25, 17.993),
            (20.0, 50.0, 1013.25, 13.873),
            (10.0, 80.0, 1013.25, 8.313),
            (0.0, 50.0, 1013.25, -2.850),
            (35.0, 40.0, 1000.0, 24.060),
            (30.0, 50.0, 900.0, 21.850),
            (30.0, 100.0, 1013.25, 30.000),
            (30.0, 0.0, 1013.25, 10.833),
        ];

        for (air_temperature_c, relative_humidity, pressure_hpa, expected) in cases {
            let actual = wet_bulb(air_temperature_c, relative_humidity, pressure_hpa)
                .expect("wet_bulb should return Ok");

            assert!(
                (actual - expected).abs() < 0.01,
                "wet_bulb({air_temperature_c}, {relative_humidity}, {pressure_hpa}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_wet_bulb_rejects_relative_humidity_below_zero() {
        let actual = wet_bulb(30.0, -1.0, 1013.25);

        assert_eq!(actual, Err(WetBulbError::RelativeHumidityOutOfRange));
    }

    #[test]
    fn test_wet_bulb_rejects_relative_humidity_above_100() {
        let actual = wet_bulb(30.0, 101.0, 1013.25);

        assert_eq!(actual, Err(WetBulbError::RelativeHumidityOutOfRange));
    }

    #[test]
    fn test_wet_bulb_rejects_invalid_pressure() {
        let actual = wet_bulb(30.0, 50.0, 0.0);

        assert_eq!(actual, Err(WetBulbError::PressureOutOfRange));
    }

    #[test]
    fn test_wet_bulb_rejects_nan_temperature() {
        let actual = wet_bulb(f64::NAN, 50.0, 1013.25);

        assert_eq!(actual, Err(WetBulbError::InvalidTemperature));
    }

    #[test]
    fn test_wet_bulb_rejects_nan_relative_humidity() {
        let actual = wet_bulb(30.0, f64::NAN, 1013.25);

        assert_eq!(actual, Err(WetBulbError::InvalidRelativeHumidity));
    }

    #[test]
    fn test_wet_bulb_rejects_nan_pressure() {
        let actual = wet_bulb(30.0, 50.0, f64::NAN);

        assert_eq!(actual, Err(WetBulbError::InvalidPressure));
    }
}
