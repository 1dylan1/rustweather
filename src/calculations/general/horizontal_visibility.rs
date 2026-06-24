use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VisibilityError {
    InvalidPressure,
    InvalidTemperature,
    InvalidCloudWater,
    InvalidRainWater,
    InvalidSnow,
    InvalidIce,
    NonPositivePressure,
    NonPositiveTemperature,
}

/// Computes horizontal meteorological visibility using the
/// Stoelinga–Warner (1999) hydrometeor extinction method.
///
/// Visibility is diagnosed from the extinction caused by cloud water,
/// rain, snow, and cloud ice in the lowest atmospheric layer.
///
/// The extinction coefficient β is computed as:
///
/// β = 144.7 Cc^0.88
///   + 2.24 Cr^0.75
///   + 327.8 Cs^0.78
///   + 10.4 Ci
///
/// where:
///
/// * `Cc` = cloud water concentration (g m⁻³)
/// * `Cr` = rain water concentration (g m⁻³)
/// * `Cs` = snow concentration (g m⁻³)
/// * `Ci` = cloud ice concentration (g m⁻³)
///
/// Visibility is then computed using Koschmieder's law:
///
/// VIS = 3.912 / β
///
/// # Type Parameters
///
/// Inputs may be any numeric type implementing [`ToPrimitive`].
///
/// # Arguments
///
/// * `pressure_pa` - Atmospheric pressure in Pascals.
/// * `temperature_k` - Air temperature in Kelvin.
/// * `cloud_water` - Cloud water mixing ratio (kg kg⁻¹).
/// * `rain_water` - Rain water mixing ratio (kg kg⁻¹).
/// * `snow` - Snow mixing ratio (kg kg⁻¹).
/// * `ice` - Cloud ice mixing ratio (kg kg⁻¹).
///
/// # Returns
///
/// Returns visibility in kilometers.
///
/// Visibility is capped at 24 km, representing effectively
/// unrestricted visibility.
///
/// # Notes
///
/// Mixing ratios are converted to concentrations using:
///
/// ```text
/// C = q × ρ
/// ```
///
/// where:
///
/// * `q` is the mixing ratio (kg kg⁻¹)
/// * `ρ` is the air density (kg m⁻³)
///
/// Air density is computed from the ideal gas law:
///
/// ```text
/// ρ = p / (Rd × T)
/// ```
///
/// where:
///
/// * `p` = pressure (Pa)
/// * `T` = temperature (K)
/// * `Rd` = 287.05 J kg⁻¹ K⁻¹
///
/// # Limitations
///
/// This method does not account for:
///
/// * Smoke or dust
/// * Aerosol haze
/// * Blowing snow
/// * Terrain obscuration
/// * Relative-humidity haze effects
///
/// Best results are obtained using the lowest atmospheric
/// model level above ground.
pub fn horizontal_visibility<P, T, C, R, S, I>(
    pressure_pa: P,
    temperature_k: T,
    cloud_water: C,
    rain_water: R,
    snow: S,
    ice: I,
) -> Result<f64, VisibilityError>
where
    P: ToPrimitive,
    T: ToPrimitive,
    C: ToPrimitive,
    R: ToPrimitive,
    S: ToPrimitive,
    I: ToPrimitive,
{
    let pressure_pa = pressure_pa
        .to_f64()
        .ok_or(VisibilityError::InvalidPressure)?;

    let temperature_k = temperature_k
        .to_f64()
        .ok_or(VisibilityError::InvalidTemperature)?;

    let cloud_water = cloud_water
        .to_f64()
        .ok_or(VisibilityError::InvalidCloudWater)?;

    let rain_water = rain_water
        .to_f64()
        .ok_or(VisibilityError::InvalidRainWater)?;

    let snow = snow.to_f64().ok_or(VisibilityError::InvalidSnow)?;

    let ice = ice.to_f64().ok_or(VisibilityError::InvalidIce)?;

    if pressure_pa <= 0.0 {
        return Err(VisibilityError::NonPositivePressure);
    }

    if temperature_k <= 0.0 {
        return Err(VisibilityError::NonPositiveTemperature);
    }

    // Air density (kg m^-3)
    let rho = pressure_pa / (287.05 * temperature_k);

    // Convert to concentrations (g m^-3)
    let cc = cloud_water * rho * 1000.0;
    let cr = rain_water * rho * 1000.0;
    let cs = snow * rho * 1000.0;
    let ci = ice * rho * 1000.0;

    // Extinction coefficient (km^-1)
    let beta = 144.7 * cc.powf(0.88) + 2.24 * cr.powf(0.75) + 327.8 * cs.powf(0.78) + 10.4 * ci;

    if beta <= 0.0 {
        return Ok(24.0);
    }

    Ok((3.912 / beta).min(24.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_visibility() {
        // TODO: Get more test cases from HRRR / grib files and compare, the two I tested looked reasonable/comparable enough for now
        let actual = horizontal_visibility(101325.0, 293.15, 0.0, 0.0, 0.0, 0.0).unwrap();
        let expected = 24.0;

        assert!(
            (actual - expected).abs() < 0.001,
            "horizontal_visibility(101325.0, 293.15, 0.0, 0.0, 0.0, 0.0) = {actual}, expected {expected}"
        );
    }
}
