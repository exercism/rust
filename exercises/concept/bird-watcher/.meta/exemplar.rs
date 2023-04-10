pub fn last_week_log() -> [i32; 7] {
    return [0, 2, 5, 3, 7, 8, 4];
}

pub fn count_today(watch_log: [i32; 7]) -> i32 {
    return watch_log[6];
}

pub fn log_today(watch_log: &mut [i32; 7]) {
    watch_log[6] += 1;
}

pub fn has_day_without_birds(watch_log: [i32; 7]) -> bool {
    for count in watch_log {
        if count == 0 {
            return true;
        }
    }

    return false;
}

pub fn tally_days(watch_log: [i32; 7], days: usize) -> i32 {
    let mut total = 0;
    let mut i = 0;

    for count in watch_log {
        if i >= days {
            break;
        }
        i += 1;

        total += count;
    }

    return total;
}

pub fn calc_busy_days(watch_log: [i32; 7]) -> u8 {
    let mut days = 0;

    for count in watch_log {
        if count >= 5 {
            days += 1;
        }
    }

    return days;
}
