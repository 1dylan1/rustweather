use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeopotentialError {
    InvalidHeight,
    HeightOutOfRange,
}

/// Computes geopotential from geometric height.
///
/// Geopotential is the potential energy per unit mass at a given height in the
/// atmosphere. It is commonly used in meteorology and has units of `m^2/s^2`.
///
/// # Type Parameters
///
/// `T` can be any numeric type that implements [`ToPrimitive`] and can be
/// converted to `f64`.
///
/// # Arguments
///
/// * `height_m` - Geometric height in meters above mean sea level.
///
/// # Returns
///
/// Returns `Ok(geopotential)` in square meters per second squared, `m^2/s^2`.
///
/// Returns an error if the height cannot be converted to `f64`, is not finite,
/// or is negative.
pub fn height_to_geopotential<T>(height_m: T) -> Result<f64, GeopotentialError>
where
    T: ToPrimitive,
{
    let height_m = height_m.to_f64().ok_or(GeopotentialError::InvalidHeight)?;

    if !height_m.is_finite() {
        return Err(GeopotentialError::InvalidHeight);
    }

    if height_m < 0.0 {
        return Err(GeopotentialError::HeightOutOfRange);
    }

    let earth_radius_m = 6_356_766.0;
    let standard_gravity = 9.806_65;

    Ok((standard_gravity * earth_radius_m * height_m) / (earth_radius_m + height_m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_to_geopotential_cases() {
        let cases = [
            // geometric height m, expected geopotential m²/s²
            (0.0, 0.0),
            (1000.0, 9805.107),
            (5000.0, 48994.712),
            (10000.0, 97912.471),
            (20000.0, 195517.851),
            (50000.0, 486505.822),
        ];

        for (height_m, expected) in cases {
            let actual =
                height_to_geopotential(height_m).expect("height_to_geopotential should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "height_to_geopotential({height_m}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_height_to_geopotential_rejects_negative_height() {
        let actual = height_to_geopotential(-1.0);

        assert_eq!(actual, Err(GeopotentialError::HeightOutOfRange));
    }
}
