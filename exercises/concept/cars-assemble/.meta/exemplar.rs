const PRODUCTION_RATE_PER_HOUR_FOR_DEFAULT_SPEED: i32 = 221;

pub fn production_rate_per_hour(speed: i32) -> f64 {
    let result = production_rate_per_hour_for_speed(speed) * success_rate(speed);

    round_to_1_decimal(result)
}

fn round_to_1_decimal(input: f64) -> f64 {
    (input * 10.0).round() / 10.0
}

fn production_rate_per_hour_for_speed(speed: i32) -> f64 {
    (PRODUCTION_RATE_PER_HOUR_FOR_DEFAULT_SPEED * speed).into()
}

pub fn working_items_per_minute(speed: i32) -> i32 {
    (production_rate_per_hour(speed) / 60.0) as i32
}

pub fn success_rate(speed: i32) -> f64 {
    if speed == 10 {
        0.77
    } else if speed == 9 {
        0.8
    } else if speed >= 5 {
        0.9
    } else if speed <= 0 {
        0.0
    } else {
        1.0
    }
}
