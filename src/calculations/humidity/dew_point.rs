use num_traits::ToPrimitive;

use crate::calculations::humidity::saturated_vapor_pressure::{
    SaturationVaporPressureError, saturation_vapor_pressure,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DewPointError {
    InvalidVaporPressure,
    VaporPressureOutOfRange,
    InvalidTemperature,
    InvalidRelHumidity,
    SaturationVaporPressureError(SaturationVaporPressureError),
}

/// Computes dew point from vapor pressure.
///
/// Dew point is the temperature air must cool to for water vapor to begin
/// condensing. This function calculates dew point from actual vapor pressure.
/// Adapted from NWS.
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

/// Computes dew point from air temperature and relative humidity.
/// Adapted from NWS.
///
/// Dew point is the temperature air must cool to for water vapor to begin
/// condensing. This function estimates dew point from air temperature and
/// relative humidity.
///
/// # Arguments
///
/// * `air_temperature_c` - Air temperature in degrees Celsius. Should be between -35<=x<=45
/// * `relative_humidity` - Relative humidity in percent, from `0.0` to `100.0`.
///
/// # Returns
///
/// Returns `Ok(dew_point_c)` in degrees Celsius.
pub fn dew_point_from_temp_and_rh<T, U>(
    air_temperature_c: T,
    relative_humidity: U,
) -> Result<f64, DewPointError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let air_temperature_c = air_temperature_c
        .to_f64()
        .ok_or(DewPointError::InvalidTemperature)?;

    let relative_humidity = relative_humidity
        .to_f64()
        .ok_or(DewPointError::InvalidVaporPressure)?;

    // rh > 100 is possible, albeit rare. However, the more likely scenario is RH was either calculated
    // incorrectly (beit in rustweather, or externally) or the data source is bad.
    if !(0.0..=100.0).contains(&relative_humidity) {
        return Err(DewPointError::InvalidRelHumidity);
    }

    let saturation_vapor_pressure_hpa = saturation_vapor_pressure(air_temperature_c)
        .map_err(DewPointError::SaturationVaporPressureError)?;
    let actual_vapor_pressure_hpa =
        ((saturation_vapor_pressure_hpa * relative_humidity) / 611.0).ln();

    return Ok(
        (237.3 * actual_vapor_pressure_hpa) / (7.5 * 10.0f64.ln() - actual_vapor_pressure_hpa)
    );
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

    #[test]
    fn test_dew_point_temp_and_rh() {
        let cases = vec![
            // temperature_c, relative_humidity, expected dew point
            (30.0, 30.0, 10.542),
            (25.0, 45.0, 12.240),
            (10.0, 55.0, 1.375),
            (45.0, 25.0, 20.450),
        ];
        for (temperature_c, relative_humidity, expected) in cases {
            let actual = dew_point_from_temp_and_rh(temperature_c, relative_humidity)
                .expect("dew point should return Ok");
            assert!(
                (actual - expected).abs() < 0.001,
                "dew_point_from_vapor_pressure({temperature_c}, {relative_humidity}) = {actual}, expected {expected}."
            );
        }
    }
}
