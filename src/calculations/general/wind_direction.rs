use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindDirectionError {
    InvalidComponent,
}

/// Computes the wind direction from u and v components.
/// Wind direction is the direction from which wind is blowing from. Wind direction increases clockwise,
/// where a north wind is 0 degrees, east is 90, south 180, and west 270.
///
/// # Type Parameters
///
/// `T` can be any numeric type that implements [`ToPrimitive`] and can be
/// converted to `f64`.
///
/// # Arguments
///
/// * `u` - meters/second value for u-direction in East-West
/// * `v` - meters/second value for v-direction in North-South
///
/// # Returns
/// * `wind_direction` 0-360 degree direction of the wind
pub fn wind_direction<T, U>(u: T, v: U) -> Result<f64, WindDirectionError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let u = u.to_f64().ok_or(WindDirectionError::InvalidComponent)?;
    let v = v.to_f64().ok_or(WindDirectionError::InvalidComponent)?;

    if u == 0.0 && v == 0.0 {
        return Ok(0.0);
    }

    return Ok((270.0 - v.atan2(u).to_degrees()).rem_euclid(360.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wind_direction_cases() {
        let cases = vec![
            // u, v, expected direction
            (4.0, 0.0, 270.0),
            (2.0, 2.0, 225.0),
            (0.0, 4.0, 180.0),
            (0.0, 0.0, 0.0),
            (1.0, -1.0, 315.0),
        ];
        for (u, v, expected) in cases {
            let actual = wind_direction(u, v).expect("wind_speed should return Ok");
            assert!(
                (actual - expected).abs() < 0.001,
                "wind_direction({u},{v}) = {actual}, expected {expected}."
            );
        }
    }
}
