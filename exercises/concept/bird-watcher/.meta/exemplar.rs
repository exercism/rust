pub fn bird_count(observation_log: Vec<u16>) -> u16 {
    let total: u16 = observation_log.iter().sum();
    total
}

pub fn birds_in_week(observation_log: Vec<u16>, week: usize) -> u16 {
    let start = (week - 1) * 7;

    // slice from vector
    let period = &observation_log[start..start + 7];
    // let period = &observation_log[start..=start + 6];
    let total: u16 = period.iter().sum();
    total
}

pub fn fix_bird_count(mut observation_log: Vec<u16>) -> Vec<u16> {
    // bird was in the garden on the first day and every other day thereafter
    // increment the reported count on odd days (even indices)
    for bird_count in observation_log.iter_mut().step_by(2) {
        *bird_count += 1;
    }
    observation_log
}

pub fn birds_today(observation_log: Vec<u8>) -> i16 {
    if let Some(birds) = observation_log.last() {
        *birds
    } else {
        -1
    }
}

pub fn day_without_birds(observation_log: Vec<u8>) -> bool {
    observation_log.contains(&0)
}

pub fn busy_days_number(observation_log: Vec<u8>) -> u8 {
    let mut days = 0;
    for birds in observation_log.iter() {
        if birds >= &5 {
            days += 1;
        }
    }
    days
}

pub fn birds_in_first_days(observation_log: Vec<u8>, days: usize) -> u8 {
    let birds = observation_log.iter().take(days).sum();
    birds
}
