use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindComponentError {
    InvalidSpeed,
    InvalidDirection,
}
/// Computes the U and V wind components.
/// u and v are horizontal components of wind speed, U is the east-west, where positive values
/// mean that wind is blowing from west to east, and negative from east to west. V is the
/// north-south, positive values mean wind is blowing from south to north, and negative is north to
/// south.
///
/// # Type Parameters
///
/// `T` and 'U' can be any numeric type that implements [`ToPrimitive`] and can be
/// converted to `f64`.
///
/// # Arguments
///
/// * `speed` wind speed in the input speed unit, such as m/s, mph, knots.
/// * `wind_direction` - degrees, where 0 is north, 90 east, 180 south, 270 west.
///
/// # Returns
///
/// * `u` - wind component in the same speed unit as `speed`. Positive values are eastward flowing.
/// * `v` - wind component in the same speed unit as `speed`. Positive vaules are northward flowing.
pub fn wind_components<T, U>(speed: T, wind_direction: U) -> Result<(f64, f64), WindComponentError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let speed = speed.to_f64().ok_or(WindComponentError::InvalidSpeed)?;
    let wind_direction = wind_direction
        .to_f64()
        .ok_or(WindComponentError::InvalidDirection)?;

    if speed < 0.0 {
        return Err(WindComponentError::InvalidSpeed);
    }

    let direction_radians = wind_direction.to_radians();

    let u = -speed * direction_radians.sin();
    let v = -speed * direction_radians.cos();
    return Ok((u, v));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wind_component_cases() {
        let sqrt_2 = 2.0f64.sqrt();

        let cases = [
            // speed, dir, expected_u, expected_v
            (4.0, 0.0, 0.0, -4.0),
            (4.0, 45.0, -4.0 / sqrt_2, -4.0 / sqrt_2),
            (4.0, 90.0, -4.0, 0.0),
            (4.0, 135.0, -4.0 / sqrt_2, 4.0 / sqrt_2),
            (25.0, 180.0, 0.0, 25.0),
            (25.0, 225.0, 25.0 / sqrt_2, 25.0 / sqrt_2),
            (25.0, 270.0, 25.0, 0.0),
            (25.0, 315.0, 25.0 / sqrt_2, -25.0 / sqrt_2),
            (10.0, 360.0, 0.0, -10.0),
        ];

        for (speed, direction, expected_u, expected_v) in cases {
            let (actual_u, actual_v) =
                wind_components(speed, direction).expect("wind_components should return Ok");

            assert!(
                (actual_u - expected_u).abs() < 0.001,
                "wind_components({speed}, {direction}) gave u={actual_u}, expected u={expected_u}"
            );

            assert!(
                (actual_v - expected_v).abs() < 0.001,
                "wind_components({speed}, {direction}) gave v={actual_v}, expected v={expected_v}"
            );
        }
    }
}
