use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindChillError {
    InvalidTemperature,
    InvalidWindSpeed,
    TemperatureTooHigh,
    WindSpeedTooLow,
}

/// Computes the Wind Chill Temperature Index from air temperature and wind speed.
///
/// This uses the National Weather Service wind chill formula:
///
/// `WCT = 35.74 + 0.6215T - 35.75V^0.16 + 0.4275TV^0.16`
///
/// where:
///
/// * `T` is air temperature in degrees Fahrenheit
/// * `V` is wind speed in miles per hour
///
/// The formula is only defined for temperatures at or below `50°F` and
/// wind speeds greater than `3 mph`.
///
/// # Type Parameters
///
/// `T` and `U` can be any numeric types that implement [`ToPrimitive`] and
/// can be converted to `f64`.
///
/// # Arguments
///
/// * `temperature_f` - Air temperature in degrees Fahrenheit.
/// * `wind_speed_mph` - Wind speed in miles per hour.
///
/// # Returns
///
/// * `wind_chill` in degrees Fahrenheit
///
/// Returns an error if either value cannot be converted to `f64`, if the
/// temperature is above `50°F`, or if the wind speed is less than or equal to
/// `3 mph`.
pub fn wind_chill<T, U>(temperature_f: T, wind_speed_mph: U) -> Result<f64, WindChillError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let temperature_f = temperature_f
        .to_f64()
        .ok_or(WindChillError::InvalidTemperature)?;

    let wind_speed_mph = wind_speed_mph
        .to_f64()
        .ok_or(WindChillError::InvalidWindSpeed)?;

    if temperature_f > 50.0 {
        return Err(WindChillError::TemperatureTooHigh);
    }

    if wind_speed_mph <= 3.0 {
        return Err(WindChillError::WindSpeedTooLow);
    }

    let wind_speed_power = wind_speed_mph.powf(0.16);

    Ok(35.74 + 0.6215 * temperature_f - 35.75 * wind_speed_power
        + 0.4275 * temperature_f * wind_speed_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wind_chill_cases() {
        let cases = [
            // temp_f, wind_mph, expected wind chill
            (5.0, 30.0, -19.074055438359636),
            (0.0, 10.0, -15.934471804166904),
            (40.0, 5.0, 36.47240485832117),
            (50.0, 4.0, 48.87024835953933),
            (-5.0, 20.0, -28.554723791794206),
        ];

        for (temperature_f, wind_speed_mph, expected) in cases {
            let actual =
                wind_chill(temperature_f, wind_speed_mph).expect("wind_chill should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "wind_chill({temperature_f}, {wind_speed_mph}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_wind_chill_temperature_too_high() {
        let actual = wind_chill(51.0, 10.0);

        assert_eq!(actual, Err(WindChillError::TemperatureTooHigh));
    }

    #[test]
    fn test_wind_chill_speed_too_low() {
        let actual = wind_chill(30.0, 3.0);

        assert_eq!(actual, Err(WindChillError::WindSpeedTooLow));
    }
}
