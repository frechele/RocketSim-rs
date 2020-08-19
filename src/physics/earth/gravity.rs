use std::f32::consts as f32_consts;

pub fn gravity(latitude: f32, altitude: f32) -> f32 {
    let phi = latitude * f32_consts::PI / 360.0;

    const G_EQUATOR: f32 = 9.7803267714;
    const K: f32 = 0.0019185138639;
    const E_SQUARE: f32 = 0.006694384442;

    let g_phi = G_EQUATOR * (1.0 + K * phi.sin().powi(2)) / (1.0 - E_SQUARE * phi.sin().powi(2)).sqrt();

    const R_EQUATOR: f32 = 6378100.0;

    g_phi * (R_EQUATOR / (R_EQUATOR + altitude)).powi(2)
}
