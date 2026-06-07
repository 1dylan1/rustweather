use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PressureToHeightError {
    InvalidPressure,
    PressureOutOfRange,
}

/// Computes approximate pressure altitude from pressure according to NWS' formula.
///
/// This converts pressure in hectopascals(millibars) to an approximate standard-atmosphere
/// height. The result is not true terrain height or model geopotential height;
/// it is the altitude corresponding to the pressure under a standard atmosphere.
///
/// # Arguments
///
/// * `pressure_hpa` - Pressure in hectopascals/millibars.
///
/// # Returns
///
/// Returns `Ok(height_ft)` in feet.
pub fn pressure_to_height<T>(pressure_hpa: T) -> Result<f64, PressureToHeightError>
where
    T: ToPrimitive,
{
    let pressure_hpa = pressure_hpa
        .to_f64()
        .ok_or(PressureToHeightError::InvalidPressure)?;

    if pressure_hpa < 0.0 {
        return Err(PressureToHeightError::PressureOutOfRange);
    }

    return Ok((1.0 - (pressure_hpa / 1013.25).powf(0.190284)) * 145366.45);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_to_height() {
        let cases = vec![
            // hpa/mb, expected height
            (1013.0, 6.825),
            (1000.0, 363.644),
            (0.0, 145366.450),
        ];
        for (pressure_hpa, expected) in cases {
            let actual = pressure_to_height(pressure_hpa).expect("height should return Ok");
            assert!(
                (actual - expected).abs() < 0.001,
                "pressure_to_height({pressure_hpa}) = {actual}, expected {expected}"
            )
        }
    }
}
