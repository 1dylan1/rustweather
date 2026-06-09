use num_traits::ToPrimitive;

use crate::calculations::humidity::{
    ActualVaporPressureError, SaturationVaporPressureError, actual_vapor_pressure,
    saturation_vapor_pressure,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MixingRatioError {
    InvalidPressure,
    InvalidTemperature,
    TemperatureOutOfRange,
}

/// Computes the actual mixing ratio from pressure and dewpoint temperature.
///
/// The actual mixing ratio is the mass of water vapor compared to the mass of
/// dry air. It describes how much water vapor is actually present in the air.
/// Adapted from NWS mixing ratio calculations.
///
/// # Type Parameters
///
/// * `T` - Numeric type for pressure that implements [`ToPrimitive`].
/// * `U` - Numeric type for dewpoint temperature that implements [`ToPrimitive`].
///
/// # Arguments
///
/// * `pressure_hpa` - Air pressure in hectopascals.
/// * `dewpoint_c` - Dewpoint temperature in degrees Celsius.
///
/// # Returns
///
/// Returns `Ok(mixing_ratio)` in grams per kilogram.
pub fn actual_mixing_ratio<T, U>(pressure_hpa: T, dewpoint_c: U) -> Result<f64, MixingRatioError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let pressure_hpa = pressure_hpa
        .to_f64()
        .ok_or(MixingRatioError::InvalidPressure)?;

    let dewpoint_c = dewpoint_c
        .to_f64()
        .ok_or(MixingRatioError::InvalidTemperature)?;

    if pressure_hpa <= 0.0 {
        return Err(MixingRatioError::InvalidPressure);
    }

    let actual_vapor_pressure_hpa = actual_vapor_pressure(dewpoint_c).map_err(|err| match err {
        ActualVaporPressureError::InvalidTemperature => MixingRatioError::InvalidTemperature,
        ActualVaporPressureError::TemperatureOutOfRange => MixingRatioError::TemperatureOutOfRange,
    })?;

    if actual_vapor_pressure_hpa >= pressure_hpa {
        return Err(MixingRatioError::InvalidPressure);
    }

    return Ok(621.97 * (actual_vapor_pressure_hpa / (pressure_hpa - actual_vapor_pressure_hpa)));
}

/// Computes the saturated mixing ratio from pressure and air temperature.
///
/// The saturated mixing ratio is the maximum amount of water vapor air can hold
/// at a given temperature and pressure before condensation begins.
/// Adapted from the NWS saturated mixing ratio calculation.
///
/// # Type Parameters
///
/// * `T` - Numeric type for pressure that implements [`ToPrimitive`].
/// * `U` - Numeric type for air temperature that implements [`ToPrimitive`].
///
/// # Arguments
///
/// * `pressure_hpa` - Air pressure in hectopascals.
/// * `temperature_c` - Air temperature in degrees Celsius.
///
/// # Returns
///
/// Returns `Ok(saturated_mixing_ratio)` in grams per kilogram.
pub fn saturated_mixing_ratio<T, U>(
    pressure_hpa: T,
    temperature_c: U,
) -> Result<f64, MixingRatioError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let pressure_hpa = pressure_hpa
        .to_f64()
        .ok_or(MixingRatioError::InvalidPressure)?;

    let temperature_c = temperature_c
        .to_f64()
        .ok_or(MixingRatioError::InvalidTemperature)?;

    if pressure_hpa <= 0.0 {
        return Err(MixingRatioError::InvalidPressure);
    }

    let saturated_vapor_pressure_hpa =
        saturation_vapor_pressure(temperature_c).map_err(|err| match err {
            SaturationVaporPressureError::InvalidTemperature => {
                MixingRatioError::InvalidTemperature
            }
            SaturationVaporPressureError::TemperatureOutOfRange => {
                MixingRatioError::TemperatureOutOfRange
            }
        })?;

    if saturated_vapor_pressure_hpa >= pressure_hpa {
        return Err(MixingRatioError::InvalidPressure);
    }

    return Ok(
        621.97 * (saturated_vapor_pressure_hpa / (pressure_hpa - saturated_vapor_pressure_hpa))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actual_mixing_ratio() {
        let cases = vec![
            // pressure hpa, dewpoint c, expected mixing ratio
            (1013.250, 15.000, 10.650),
            (800.000, 25.000, 25.650),
            (1050.250, 1.000, 3.914),
        ];
        for (pressure_hpa, dewpoint_c, expected) in cases {
            let actual = actual_mixing_ratio(pressure_hpa, dewpoint_c)
                .expect("actual mixing ratio should return Ok");
            assert!(
                (actual - expected).abs() < 0.001,
                "actual_mixing_ratio({pressure_hpa}, {dewpoint_c}) = {actual}, expected {expected}."
            );
        }
    }

    #[test]
    fn test_saturated_mixing_ratio() {
        let cases = vec![
            // pressure hpa, temperature c, expected mixing ratio
            (1013.250, 15.000, 10.639),
            (800.000, 25.000, 25.640),
            (1050.250, 1.000, 3.915),
        ];
        for (pressure_hpa, temperature_c, expected) in cases {
            let actual = saturated_mixing_ratio(pressure_hpa, temperature_c)
                .expect("saturated mixing ratio should return Ok");
            assert!(
                (actual - expected).abs() < 0.001,
                "saturated_mixing_ratio({pressure_hpa}, {temperature_c}) = {actual}, expected {expected}."
            );
        }
    }

    #[test]
    fn test_actual_mixing_ratio_rejects_zero_pressure() {
        let actual = actual_mixing_ratio(0.0, 15.0);

        assert_eq!(actual, Err(MixingRatioError::InvalidPressure));
    }

    #[test]
    fn test_actual_mixing_ratio_rejects_negative_pressure() {
        let actual = actual_mixing_ratio(-100.0, 15.0);

        assert_eq!(actual, Err(MixingRatioError::InvalidPressure));
    }

    #[test]
    fn test_actual_mixing_ratio_rejects_out_of_range_dewpoint_low() {
        let actual = actual_mixing_ratio(1013.25, -36.0);

        assert_eq!(actual, Err(MixingRatioError::TemperatureOutOfRange));
    }
}
