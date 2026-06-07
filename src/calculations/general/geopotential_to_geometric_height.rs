use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeopotentialError {
    InvalidHeight,
    HeightOutOfRange,
    InvalidGeopotential,
    GeopotentialOutOfRange,
}

/// Computes geometric height from geopotential.
///
/// Geopotential is the potential energy per unit mass at a given height in the
/// atmosphere. This function converts geopotential in `m^2/s^2` back to geometric
/// height in meters.
///
/// # Type Parameters
///
/// `T` can be any numeric type that implements [`ToPrimitive`] and can be
/// converted to `f64`.
///
/// # Arguments
///
/// * `geopotential` - Geopotential in square meters per second squared, `m^2/s^2`.
///
/// # Returns
///
/// Returns `Ok(height_m)` as geometric height in meters.
///
/// Returns an error if geopotential cannot be converted to `f64`, is not finite,
/// is negative, or is outside the supported range.
pub fn geopotential_to_geometric_height<T>(geopotential: T) -> Result<f64, GeopotentialError>
where
    T: ToPrimitive,
{
    let geopotential = geopotential
        .to_f64()
        .ok_or(GeopotentialError::InvalidGeopotential)?;

    if !geopotential.is_finite() {
        return Err(GeopotentialError::InvalidGeopotential);
    }

    if geopotential < 0.0 {
        return Err(GeopotentialError::GeopotentialOutOfRange);
    }

    let earth_radius_m = 6_356_766.0;
    let standard_gravity = 9.806_65;

    let geopotential_height_m = geopotential / standard_gravity;

    if geopotential_height_m >= earth_radius_m {
        return Err(GeopotentialError::GeopotentialOutOfRange);
    }

    return Ok((earth_radius_m * geopotential_height_m) / (earth_radius_m - geopotential_height_m));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geopotential_to_geometric_height_cases() {
        let cases = [
            // geopotential m^2/s^2, expected geometric height m
            (0.0, 0.0),
            (9805.107, 1000.0),
            (48994.712, 5000.0),
            (97912.471, 10000.0),
            (195517.851, 20000.0),
            (486505.822, 50000.0),
        ];

        for (geopotential, expected) in cases {
            let actual = geopotential_to_geometric_height(geopotential)
                .expect("geopotential_to_geometric_height should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "geopotential_to_geometric_height({geopotential}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_geopotential_to_geometric_height_rejects_negative_geopotential() {
        let actual = geopotential_to_geometric_height(-1.0);

        assert_eq!(actual, Err(GeopotentialError::GeopotentialOutOfRange));
    }

    #[test]
    fn test_geopotential_to_geometric_height_rejects_nan() {
        let actual = geopotential_to_geometric_height(f64::NAN);

        assert_eq!(actual, Err(GeopotentialError::InvalidGeopotential));
    }

    #[test]
    fn test_geopotential_to_geometric_height_rejects_infinity() {
        let actual = geopotential_to_geometric_height(f64::INFINITY);

        assert_eq!(actual, Err(GeopotentialError::InvalidGeopotential));
    }

    #[test]
    fn test_geopotential_to_geometric_height_rejects_out_of_range_geopotential() {
        let earth_radius_m = 6_356_766.0;
        let standard_gravity = 9.806_65;

        let actual = geopotential_to_geometric_height(earth_radius_m * standard_gravity);

        assert_eq!(actual, Err(GeopotentialError::GeopotentialOutOfRange));
    }
}
