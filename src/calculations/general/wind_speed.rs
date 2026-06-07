use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindSpeedError {
    InvalidComponent,
}

/// Computes the wind speed from u and v components.
/// Wind speed is the rate air moves horizontally.
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
/// * `wind_speed` meters/second
pub fn wind_speed<T>(u: T, v: T) -> Result<f64, WindSpeedError>
where
    T: ToPrimitive,
{
    let u = u.to_f64().ok_or(WindSpeedError::InvalidComponent)?;
    let v = v.to_f64().ok_or(WindSpeedError::InvalidComponent)?;

    return Ok((u.powi(2) + v.powi(2)).sqrt());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wind_speed_cases() {
        let cases = vec![
            // u, v, expected speed
            (4.0, 0.0, 4.0),
            (2.0, 2.0, 2.0f64.sqrt() * 2.0),
            (0.0, 4.0, 4.0),
            (0.0, 0.0, 0.0),
        ];
        for (u, v, expected) in cases {
            let actual = wind_speed(u, v).expect("wind_speed should return Ok");
            assert!(
                (actual - expected).abs() < 0.001,
                "wind_speed({u},{v}) = {actual}, expected {expected}."
            );
        }
    }
}
