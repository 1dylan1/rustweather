use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrosswindError {
    InvalidDirection,
    InvalidSpeed,
    InvalidHeading,
}

/// Computes the headwind and crosswind components of the wind relative to a heading,
/// such as a runway or aircraft heading.
///
/// The angle between the wind and the heading is `θ = wind_direction - heading`, and:
///
/// `headwind = speed * cos(θ)`
///
/// `crosswind = speed * sin(θ)`
///
/// # Type Parameters
///
/// `T`, `U`, and `V` can be any numeric types that implement [`ToPrimitive`] and can be
/// converted to `f64`.
///
/// # Arguments
///
/// * `wind_direction` - meteorological degrees the wind is blowing from, where 0 is north,
///   90 east, 180 south, 270 west.
/// * `wind_speed` - wind speed in the input speed unit, such as m/s, mph, knots.
/// * `heading` - heading in degrees, such as a runway heading (e.g. 360 for runway 36).
///
/// # Returns
///
/// * `headwind` - in the same speed unit as `wind_speed`. Positive values are a headwind,
///   negative values are a tailwind.
/// * `crosswind` - in the same speed unit as `wind_speed`. Positive values are wind from the
///   right of the heading, negative values are wind from the left.
pub fn crosswind_components<T, U, V>(
    wind_direction: T,
    wind_speed: U,
    heading: V,
) -> Result<(f64, f64), CrosswindError>
where
    T: ToPrimitive,
    U: ToPrimitive,
    V: ToPrimitive,
{
    let wind_direction = wind_direction
        .to_f64()
        .ok_or(CrosswindError::InvalidDirection)?;
    let wind_speed = wind_speed.to_f64().ok_or(CrosswindError::InvalidSpeed)?;
    let heading = heading.to_f64().ok_or(CrosswindError::InvalidHeading)?;

    if !wind_direction.is_finite() {
        return Err(CrosswindError::InvalidDirection);
    }

    if !wind_speed.is_finite() || wind_speed < 0.0 {
        return Err(CrosswindError::InvalidSpeed);
    }

    if !heading.is_finite() {
        return Err(CrosswindError::InvalidHeading);
    }

    let angle_radians = (wind_direction - heading).to_radians();

    let headwind = wind_speed * angle_radians.cos();
    let crosswind = wind_speed * angle_radians.sin();
    return Ok((headwind, crosswind));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crosswind_components_cases() {
        let half_sqrt_2 = 2.0f64.sqrt() / 2.0;

        let cases = [
            // wind direction, wind speed, heading, expected headwind, expected crosswind
            (360.0, 10.0, 360.0, 10.0, 0.0),
            (180.0, 10.0, 360.0, -10.0, 0.0),
            (90.0, 10.0, 360.0, 0.0, 10.0),
            (270.0, 10.0, 360.0, 0.0, -10.0),
            (45.0, 20.0, 0.0, 20.0 * half_sqrt_2, 20.0 * half_sqrt_2),
            (300.0, 15.0, 270.0, 15.0 * 30f64.to_radians().cos(), 7.5),
            (
                10.0,
                12.0,
                350.0,
                12.0 * 20f64.to_radians().cos(),
                12.0 * 20f64.to_radians().sin(),
            ),
            (0.0, 0.0, 90.0, 0.0, 0.0),
        ];

        for (wind_direction, wind_speed, heading, expected_headwind, expected_crosswind) in cases {
            let (actual_headwind, actual_crosswind) =
                crosswind_components(wind_direction, wind_speed, heading)
                    .expect("crosswind_components should return Ok");

            assert!(
                (actual_headwind - expected_headwind).abs() < 0.001,
                "crosswind_components({wind_direction}, {wind_speed}, {heading}) gave headwind={actual_headwind}, expected {expected_headwind}"
            );

            assert!(
                (actual_crosswind - expected_crosswind).abs() < 0.001,
                "crosswind_components({wind_direction}, {wind_speed}, {heading}) gave crosswind={actual_crosswind}, expected {expected_crosswind}"
            );
        }
    }

    #[test]
    fn test_crosswind_components_negative_speed() {
        let actual = crosswind_components(90.0, -5.0, 360.0);

        assert_eq!(actual, Err(CrosswindError::InvalidSpeed));
    }

    #[test]
    fn test_crosswind_components_rejects_nan() {
        assert_eq!(
            crosswind_components(f64::NAN, 5.0, 360.0),
            Err(CrosswindError::InvalidDirection)
        );
        assert_eq!(
            crosswind_components(90.0, 5.0, f64::NAN),
            Err(CrosswindError::InvalidHeading)
        );
    }
}
