use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeopotentialHeightError {
    InvalidHeight,
    HeightOutOfRange,
}

/// Computes geopotential height from geometric height.
///
/// Geopotential height is geometric height adjusted for the decrease of gravity
/// with altitude. It is commonly used in meteorology as a vertical coordinate.
///
/// Equation adapted from "Practical Meterology: An Algebra-based Survey of Atmospheric Science"
/// 2017, by Roland Stull, page 11.
///
/// # Arguments
///
/// * `geometric_height_m` - Geometric height in meters above mean sea level.
///
/// # Returns
///
/// Returns `Ok(geopotential_height)` in geopotential meters.
pub fn geometric_to_geopotential_height<T>(
    geometric_height: T,
) -> Result<f64, GeopotentialHeightError>
where
    T: ToPrimitive,
{
    let geometric_height_m = geometric_height
        .to_f64()
        .ok_or(GeopotentialHeightError::InvalidHeight)?;

    if geometric_height_m < 0.0 {
        return Err(GeopotentialHeightError::HeightOutOfRange);
    }

    let earth_radius_m = 6_356_766.0;

    return Ok((earth_radius_m * geometric_height_m) / (earth_radius_m + geometric_height_m));
}

#[test]
fn test_geometric_to_geopotential_height() {
    let cases = [
        // geometric height m, expected geopotential height m
        (0.0, 0.0),
        (1000.0, 999.842),
        (5000.0, 4996.070),
        (10000.0, 9984.293),
        (20000.0, 19937.272),
        (50000.0, 49609.787),
    ];

    for (geometric_height_m, expected) in cases {
        let actual = geometric_to_geopotential_height(geometric_height_m)
            .expect("geometric_to_geopotential_height should return Ok");

        assert!(
            (actual - expected).abs() < 0.001,
            "geometric_to_geopotential_height({geometric_height_m}) = {actual}, expected {expected}"
        );
    }
}
