use num_traits::ToPrimitive;

use crate::calculations::humidity::{
    ActualVaporPressureError, SaturationVaporPressureError, actual_vapor_pressure,
    saturation_vapor_pressure,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelativeHumidityError {
    InvalidActualMixingRatio,
    InvalidSaturatedMixingRatio,
    TemperatureOutOfRange,
    InvalidTemperature,
    DewPointGreatherThanTemp,
}

/// Computes relative humidity from actual and saturated mixing ratio.
///
/// Relative humidity is the ratio of the actual amount of water vapor in the air
/// to the maximum amount of water vapor the air can hold at the same temperature
/// and pressure.
/// Uses the August-Roche-Magnus approach.
///
/// # Type Parameters
///
/// * `T` - Numeric type for actual mixing ratio that implements [`ToPrimitive`].
/// * `U` - Numeric type for saturated mixing ratio that implements [`ToPrimitive`].
///
/// # Arguments
///
/// * `temp_c` - temperature in degrees Celsius.
/// * `dewpoint_c` - dew point in degrees Celsius.
///
/// # Returns
///
/// Returns `Ok(relative_humidity)` as a percentage. (0-100)
pub fn relative_humidity_from_temp_dewpoint<T, U>(
    temp_c: T,
    dewpoint_c: U,
) -> Result<f64, RelativeHumidityError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let temp_c = temp_c
        .to_f64()
        .ok_or(RelativeHumidityError::InvalidActualMixingRatio)?;

    let dewpoint_c = dewpoint_c
        .to_f64()
        .ok_or(RelativeHumidityError::InvalidSaturatedMixingRatio)?;

    if dewpoint_c == temp_c {
        return Ok(100.0);
    }

    if dewpoint_c > temp_c {
        return Err(RelativeHumidityError::DewPointGreatherThanTemp);
    }

    let saturated_vapor_pressure = saturation_vapor_pressure(temp_c).map_err(|err| match err {
        SaturationVaporPressureError::TemperatureOutOfRange => {
            RelativeHumidityError::TemperatureOutOfRange
        }
        SaturationVaporPressureError::InvalidTemperature => {
            RelativeHumidityError::InvalidTemperature
        }
    })?;

    let actual_vapor_pressure = actual_vapor_pressure(dewpoint_c).map_err(|err| match err {
        ActualVaporPressureError::InvalidTemperature => RelativeHumidityError::InvalidTemperature,
        ActualVaporPressureError::TemperatureOutOfRange => {
            RelativeHumidityError::TemperatureOutOfRange
        }
    })?;

    return Ok((actual_vapor_pressure / saturated_vapor_pressure) * 100.0);
}

/// Computes relative humidity from actual and saturated mixing ratio.
///
/// Relative humidity is the ratio of the actual amount of water vapor in the air
/// to the maximum amount of water vapor the air can hold at the same temperature
/// and pressure.
/// Adapted from NWS mixing ratio calculations.
///
/// # Type Parameters
///
/// * `T` - Numeric type for actual mixing ratio that implements [`ToPrimitive`].
/// * `U` - Numeric type for saturated mixing ratio that implements [`ToPrimitive`].
///
/// # Arguments
///
/// * `actual_mixing_ratio` - Actual mixing ratio in grams per kilogram.
/// * `saturated_mixing_ratio` - Saturated mixing ratio in grams per kilogram.
///
/// # Returns
///
/// Returns `Ok(relative_humidity)` as a percentage. (0-100)
pub fn relative_humidity_from_mixing_ratio<T, U>(
    actual_mixing_ratio: T,
    saturated_mixing_ratio: U,
) -> Result<f64, RelativeHumidityError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let actual_mixing_ratio = actual_mixing_ratio
        .to_f64()
        .ok_or(RelativeHumidityError::InvalidActualMixingRatio)?;

    let saturated_mixing_ratio = saturated_mixing_ratio
        .to_f64()
        .ok_or(RelativeHumidityError::InvalidSaturatedMixingRatio)?;

    if !actual_mixing_ratio.is_finite() || actual_mixing_ratio < 0.0 {
        return Err(RelativeHumidityError::InvalidActualMixingRatio);
    }

    if !saturated_mixing_ratio.is_finite() || saturated_mixing_ratio <= 0.0 {
        return Err(RelativeHumidityError::InvalidSaturatedMixingRatio);
    }

    return Ok((actual_mixing_ratio / saturated_mixing_ratio) * 100.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_humidity_from_mixing_ratio_cases() {
        let cases = [
            // actual mixing ratio, saturated mixing ratio, expected RH %
            (5.0, 10.0, 50.0),
            (10.0, 10.0, 100.0),
            (7.5, 15.0, 50.0),
            (3.914, 7.828, 50.0),
            (10.650, 14.200, 75.0),
        ];

        for (actual_mixing_ratio, saturated_mixing_ratio, expected) in cases {
            let actual =
                relative_humidity_from_mixing_ratio(actual_mixing_ratio, saturated_mixing_ratio)
                    .expect("relative_humidity_from_mixing_ratio should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "relative_humidity_from_mixing_ratio({actual_mixing_ratio}, {saturated_mixing_ratio}) = {actual}, expected {expected}."
            );
        }
    }

    #[test]
    fn test_relative_humidity_rejects_negative_actual_mixing_ratio() {
        let actual = relative_humidity_from_mixing_ratio(-1.0, 10.0);

        assert_eq!(actual, Err(RelativeHumidityError::InvalidActualMixingRatio));
    }

    #[test]
    fn test_relative_humidity_rejects_nan_actual_mixing_ratio() {
        let actual = relative_humidity_from_mixing_ratio(f64::NAN, 10.0);

        assert_eq!(actual, Err(RelativeHumidityError::InvalidActualMixingRatio));
    }

    #[test]
    fn test_relative_humidity_rejects_infinite_actual_mixing_ratio() {
        let actual = relative_humidity_from_mixing_ratio(f64::INFINITY, 10.0);

        assert_eq!(actual, Err(RelativeHumidityError::InvalidActualMixingRatio));
    }

    #[test]
    fn test_relative_humidity_rejects_zero_saturated_mixing_ratio() {
        let actual = relative_humidity_from_mixing_ratio(5.0, 0.0);

        assert_eq!(
            actual,
            Err(RelativeHumidityError::InvalidSaturatedMixingRatio)
        );
    }

    #[test]
    fn test_relative_humidity_rejects_negative_saturated_mixing_ratio() {
        let actual = relative_humidity_from_mixing_ratio(5.0, -10.0);

        assert_eq!(
            actual,
            Err(RelativeHumidityError::InvalidSaturatedMixingRatio)
        );
    }

    #[test]
    fn test_relative_humidity_rejects_nan_saturated_mixing_ratio() {
        let actual = relative_humidity_from_mixing_ratio(5.0, f64::NAN);

        assert_eq!(
            actual,
            Err(RelativeHumidityError::InvalidSaturatedMixingRatio)
        );
    }

    #[test]
    fn test_relative_humidity_rejects_infinite_saturated_mixing_ratio() {
        let actual = relative_humidity_from_mixing_ratio(5.0, f64::INFINITY);

        assert_eq!(
            actual,
            Err(RelativeHumidityError::InvalidSaturatedMixingRatio)
        );
    }

    #[test]
    fn test_relative_humidity_from_temp_dewpoint() {
        // temp_c, dewpoint_c, expected rh
        let cases = vec![
            (25.000, 20.000, 73.843),
            (0.000, 0.000, 100.000),
            (23.456, 17.720, 70.262),
        ];
        for (temp_c, dewpoint_c, expected) in cases {
            let actual = relative_humidity_from_temp_dewpoint(temp_c, dewpoint_c)
                .expect("relative_humidity_from_temp_dewpoint should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "relative_humidity_from_temp_dewpoint({temp_c}, {dewpoint_c}) = {actual}, expected {expected}."
            )
        }
    }
}
