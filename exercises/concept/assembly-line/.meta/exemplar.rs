const PRODUCTION_RATE_DEFAULT: f64 = 221.;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        s @ 1..=4 => PRODUCTION_RATE_DEFAULT * s as f64,
        s @ 5..=8 => PRODUCTION_RATE_DEFAULT * s as f64 * 0.9,
        s @ 9..=10 => PRODUCTION_RATE_DEFAULT * s as f64 * 0.77,
        _ => panic!("Speed should be a value from 0 to 10"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
