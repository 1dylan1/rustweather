use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeightToPressureError {
    InvalidHeight,
    HeightOutOfRange,
}

/// Computes pressure from geometric height using the International Standard
/// Atmosphere lapse-rate formula.
///
/// This estimates atmospheric pressure at a given height assuming a standard
/// atmosphere. It is commonly used for pressure-height approximations in the
/// lower atmosphere.
///
/// # Type Parameters
///
/// `T` can be any numeric type that implements [`ToPrimitive`] and can be
/// converted to `f64`.
///
/// # Arguments
///
/// * `height_m` - Geometric height in meters.
///
/// # Returns
///
/// Returns `Ok(pressure_hpa)` in hectopascals.
///
/// Returns an error if the height cannot be converted to `f64`, is not finite,
/// is negative, or is outside the valid lower-atmosphere range.
pub fn height_to_pressure<T>(height_m: T) -> Result<f64, HeightToPressureError>
where
    T: ToPrimitive,
{
    let height_m = height_m
        .to_f64()
        .ok_or(HeightToPressureError::InvalidHeight)?;

    if !height_m.is_finite() {
        return Err(HeightToPressureError::InvalidHeight);
    }

    if height_m < 0.0 {
        return Err(HeightToPressureError::HeightOutOfRange);
    }

    // Troposphere portion of the ISA lapse-rate formula.
    // Above this, the temperature profile changes and this formula should not
    // be used as-is. TODO: Add the Isothermal layer formula later if people request it
    if height_m > 11_000.0 {
        return Err(HeightToPressureError::HeightOutOfRange);
    }

    let sea_level_pressure_hpa = 1013.25;
    let sea_level_temperature_k = 288.15;
    let lapse_rate_k_per_m = 0.0065;
    let gravity = 9.806_65;
    let molar_mass_air = 0.028_964_4;
    let gas_constant = 8.314_459_8;

    let exponent = (gravity * molar_mass_air) / (gas_constant * lapse_rate_k_per_m);

    return Ok(sea_level_pressure_hpa
        * (1.0 - (lapse_rate_k_per_m * height_m) / sea_level_temperature_k).powf(exponent));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_to_pressure_cases() {
        let cases = [
            // height_m, expected pressure_hpa
            (0.0, 1013.25),
            (1000.0, 898.747),
            (5000.0, 540.205),
            (10000.0, 264.368),
            (11000.0, 226.326),
        ];

        for (height_m, expected) in cases {
            let actual = height_to_pressure(height_m).expect("height_to_pressure should return Ok");

            assert!(
                (actual - expected).abs() < 0.001,
                "height_to_pressure({height_m}) = {actual}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_height_to_pressure_rejects_negative_height() {
        let actual = height_to_pressure(-1.0);

        assert_eq!(actual, Err(HeightToPressureError::HeightOutOfRange));
    }

    #[test]
    fn test_height_to_pressure_rejects_height_above_troposphere_limit() {
        let actual = height_to_pressure(11001.0);

        assert_eq!(actual, Err(HeightToPressureError::HeightOutOfRange));
    }
}
