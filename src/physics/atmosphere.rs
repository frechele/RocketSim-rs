const GEOPOTENTIAL_HEIGHTS: [f32; 9] = [-610.0, 11000.0, 20000.0, 32000.0, 47000.0, 51000.0, 71000.0, 84852.0, 90000.0]; // meter
const TEMPERATURES: [f32; 9] = [292.15, 216.65, 216.65, 228.65, 270.65, 270.65, 214.65, 186.87, 186.87]; // kelvin
const PRESSURES: [f32; 9] = [108900.0, 22632.0, 5474.9, 868.02, 110.91, 66.939, 3.9564, 0.3734, 0.000064]; // pascals

pub struct AtmosphereDynamic {
    pub temperature: f32,
    pub pressure: f32,
    pub density: f32
}

fn get_level(altitude: f32) -> usize {
    for (level, val) in GEOPOTENTIAL_HEIGHTS.iter().enumerate() {
        if altitude < *val {
            return level - 1;
        }
    }

    GEOPOTENTIAL_HEIGHTS.len() - 2
}

pub fn atmosisa(altitude: f32) -> AtmosphereDynamic {
    if altitude < -5000.0 || altitude > 86000.0 {
        panic!("altitude must be between -5000.0 and 86000.0");
    }

    const G: f32 = 9.80665; // gravity acceleration (m/s^2)
    const M: f32 = 0.0289647; // Molar mass of dry air (kg/mol)
    const R: f32 = 8.31446; // Ideal gas constant (J/K/mol)
    const RS: f32 = 287.058; // Specific gas constants for dry air (J/kg/K)

    let level = get_level(altitude);

    let delta = altitude - GEOPOTENTIAL_HEIGHTS[level];
    let lapse = (TEMPERATURES[level+1] - TEMPERATURES[level]) / (GEOPOTENTIAL_HEIGHTS[level+1] - GEOPOTENTIAL_HEIGHTS[level]);

    let temperature = TEMPERATURES[level] + delta * lapse;
    let pressure: f32 = if lapse.abs() < 1e-10 {
        PRESSURES[level] * (-G * M * delta / (R * temperature)).exp()
    } else {
        PRESSURES[level] * (1.0 + lapse * delta / TEMPERATURES[level]).powf(-G * M / (R * lapse))
    };

    let density: f32 = pressure / (RS * temperature);

    AtmosphereDynamic {
        temperature,
        pressure,
        density
    }
}
