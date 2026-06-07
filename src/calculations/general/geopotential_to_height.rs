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

    Ok((earth_radius_m * geopotential_height_m) / (earth_radius_m - geopotential_height_m))
}
