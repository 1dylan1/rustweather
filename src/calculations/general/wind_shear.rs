use num_traits::ToPrimitive;

use crate::calculations::general::wind_components;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindShearError {
    InvalidComponent,
}

/// Computes wind shear from two sets of U/V wind components.
///
/// Wind shear is the vector difference between two wind vectors:
///
/// # Type Parameters
///
/// * `T` - Numeric type for lower level u component that implements [`ToPrimitive`].
/// * `U` - Numeric type for lower level v that implements [`ToPrimitive`].
/// * `V` - Numeric type for upper level u that implements [`ToPrimitive`].
/// * `W` - Numeric type for upper level v that implements [`ToPrimitive`].
///
/// # Arguments
///
/// * `lower_u` - lower level wind component U.
/// * `lower_v` - lower level wind component V.
/// * `upper_u` - upper level wind component U.
/// * `upper_v` - upper level wind component V.
///
/// # Returns
///
/// Wind shear magnitude in the same units as the input components.
pub fn wind_shear_from_components<T, U, V, W>(
    lower_u: T,
    lower_v: U,
    upper_u: V,
    upper_v: W,
) -> Result<f64, WindShearError>
where
    T: ToPrimitive,
    U: ToPrimitive,
    V: ToPrimitive,
    W: ToPrimitive,
{
    let lower_u = lower_u.to_f64().ok_or(WindShearError::InvalidComponent)?;

    let lower_v = lower_v.to_f64().ok_or(WindShearError::InvalidComponent)?;

    let upper_u = upper_u.to_f64().ok_or(WindShearError::InvalidComponent)?;

    let upper_v = upper_v.to_f64().ok_or(WindShearError::InvalidComponent)?;

    if !lower_u.is_finite() || !lower_v.is_finite() || !upper_u.is_finite() || !upper_v.is_finite()
    {
        return Err(WindShearError::InvalidComponent);
    }

    Ok(((upper_u - lower_u).powi(2) + (upper_v - lower_v).powi(2)).sqrt())
}

/// Computes wind shear from wind direction and speed at two levels.
///
/// Wind shear is calculated by converting both winds into U/V components
/// and then taking the magnitude of the vector difference between them.
///
/// # Type Parameters
///
/// * `T` - Numeric type for the lower-level wind direction that implements [`ToPrimitive`].
/// * `U` - Numeric type for the lower-level wind speed that implements [`ToPrimitive`].
/// * `V` - Numeric type for the upper-level wind direction that implements [`ToPrimitive`].
/// * `W` - Numeric type for the upper-level wind speed that implements [`ToPrimitive`].
///
/// # Arguments
///
/// * `lower_direction_deg` - lower level wind direction in meteorological degrees.
/// * `lower_speed` - lower level wind speed.
/// * `upper_direction_deg` - upper level wind direction in meteorological degrees.
/// * `upper_speed` - upper level wind speed.
///
/// # Returns
///
/// Wind shear magnitude in the same units as the input wind speeds.
pub fn wind_shear_from_direction_speed<T, U, V, W>(
    lower_direction_deg: T,
    lower_speed: U,
    upper_direction_deg: V,
    upper_speed: W,
) -> Result<f64, WindShearError>
where
    T: ToPrimitive,
    U: ToPrimitive,
    V: ToPrimitive,
    W: ToPrimitive,
{
    let (lower_u, lower_v) = wind_components(lower_speed, lower_direction_deg)
        .map_err(|_| WindShearError::InvalidComponent)?;

    let (upper_u, upper_v) = wind_components(upper_speed, upper_direction_deg)
        .map_err(|_| WindShearError::InvalidComponent)?;

    return wind_shear_from_components(lower_u, lower_v, upper_u, upper_v);
}

/// Computes bulk wind shear between two atmospheric levels.
///
/// Bulk shear is the magnitude of the vector wind difference between
/// a lower level and an upper level.
///
/// Common applications include:
///
/// * Surface–1 km bulk shear
/// * Surface–3 km bulk shear
/// * Surface–6 km bulk shear
///
/// # Type Parameters
///
/// * `T` - Numeric type for the lower-level U component that implements [`ToPrimitive`].
/// * `U` - Numeric type for the lower-level V component that implements [`ToPrimitive`].
/// * `V` - Numeric type for the upper-level U component that implements [`ToPrimitive`].
/// * `W` - Numeric type for the upper-level V component that implements [`ToPrimitive`].
///
/// # Arguments
///
/// * `lower_u` - lower level wind component U.
/// * `lower_v` - lower level wind component V.
/// * `upper_u` - upper level wind component U.
/// * `upper_v` - upper level wind component V.
///
/// # Returns
///
/// Bulk wind shear magnitude in the same units as the input components.
pub fn bulk_shear<T, U, V, W>(
    lower_u: T,
    lower_v: U,
    upper_u: V,
    upper_v: W,
) -> Result<f64, WindShearError>
where
    T: ToPrimitive,
    U: ToPrimitive,
    V: ToPrimitive,
    W: ToPrimitive,
{
    wind_shear_from_components(lower_u, lower_v, upper_u, upper_v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wind_shear_from_components() {
        let shear = wind_shear_from_components(0.0, 10.0, 50.0, 0.0).unwrap();

        assert!((shear - 50.990195).abs() < 0.001);
    }

    #[test]
    fn test_wind_shear_from_direction_speed() {
        let shear = wind_shear_from_direction_speed(180.0, 10.0, 270.0, 50.0)
            .expect("wind shear should return Ok");

        assert!((shear - 50.990195).abs() < 0.001);
    }

    #[test]
    fn test_bulk_shear() {
        let shear = bulk_shear(0.0, 10.0, 50.0, 0.0).unwrap();

        assert!((shear - 50.990195).abs() < 0.001);
    }

    #[test]
    fn test_wind_shear_rejects_nan_component() {
        let result = wind_shear_from_components(f64::NAN, 0.0, 0.0, 0.0);

        assert_eq!(result, Err(WindShearError::InvalidComponent));
    }
}
