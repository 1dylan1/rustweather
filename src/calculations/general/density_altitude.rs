use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DensityAltitudeError {
    InvalidPressureAltitude,
    InvalidTemperature,
}

/// Computes density altitude from pressure altitude and outside air temperature.
///
/// Density altitude is pressure altitude corrected for temperature deviation from the
/// International Standard Atmosphere (ISA). It represents the altitude in the standard
/// atmosphere at which the air density would be equal to the actual air density at the
/// location, and is commonly used to express aircraft and atmospheric performance.
///
/// Uses the standard ISA temperature lapse rate of 1.98 degrees C per 1000 feet, with a
/// sea-level standard temperature of 15 degrees C.
///
/// Formula adapted from the U.S. FAA "Pilot's Handbook of Aeronautical Knowledge",
/// Chapter 11: Weight and Balance.
///
/// # Arguments
///
/// * `pressure_altitude_ft` - Pressure altitude in feet.
/// * `temperature_c` - Outside air temperature in degrees Celsius.
///
/// # Returns
///
/// Returns `Ok(density_altitude)` in feet.
pub fn density_altitude<T, U>(
    pressure_altitude_ft: T,
    temperature_c: U,
) -> Result<f64, DensityAltitudeError>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let pressure_altitude_ft = pressure_altitude_ft
        .to_f64()
        .ok_or(DensityAltitudeError::InvalidPressureAltitude)?;
    let temperature_c = temperature_c
        .to_f64()
        .ok_or(DensityAltitudeError::InvalidTemperature)?;

    let isa_lapse_rate_c_per_1000ft = 1.98;
    let isa_sea_level_temp_c = 15.0;

    let isa_temp_c =
        isa_sea_level_temp_c - (isa_lapse_rate_c_per_1000ft * (pressure_altitude_ft / 1000.0));

    return Ok(pressure_altitude_ft + 118.8 * (temperature_c - isa_temp_c));
}

#[test]
fn test_density_altitude() {
    let cases = [
        // pressure altitude ft, temperature C, expected density altitude ft
        (0.0, 15.0, 0.0),
        (5000.0, 25.0, 7364.12),
        (8000.0, 30.0, 11663.792),
        (2000.0, 10.0, 1876.448),
        (10000.0, -5.0, 9976.24),
    ];
    for (pressure_altitude_ft, temperature_c, expected) in cases {
        let actual = density_altitude(pressure_altitude_ft, temperature_c)
            .expect("density_altitude should return Ok");
        assert!(
            (actual - expected).abs() < 0.001,
            "density_altitude({pressure_altitude_ft}, {temperature_c}) = {actual}, expected {expected}"
        );
    }
}
