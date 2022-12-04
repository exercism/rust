// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn bird_count(observation_log: Vec<u16>) -> u16 {
    unimplemented!("calculate total number of birds: {:?}", observation_log)
}

pub fn birds_in_week(observation_log: Vec<u16>, week: usize) -> u16 {
    unimplemented!(
        "using the given data {:?}, calculate total birds in the given week {}",
        observation_log,
        week
    )
}

pub fn fix_bird_count(mut observation_log: Vec<u16>) -> Vec<u16> {
    unimplemented!(
        "increment the count of birds by 1 on odd days in the given data {:?}",
        observation_log
    )
}

pub fn birds_today(observation_log: Vec<u8>) -> i16 {
    unimplemented!(
        "retrieve the number of birds that visited the garden today {:?}",
        observation_log
    )
}

pub fn day_without_birds(observation_log: Vec<u8>) -> bool {
    unimplemented!(
        "calculate the number of days in which no bird visited the garden {:?}",
        observation_log
    )
}

pub fn busy_days_number(observation_log: Vec<u8>) -> u8 {
    unimplemented!(
        "retrieve all days in which 5 or more birds visited the garden {:?}",
        observation_log
    )
}

pub fn birds_in_first_days(observation_log: Vec<u8>, days: usize) -> u8 {
    unimplemented!(
        "using the given data {:?}, calculate total birds in the first {} days",
        observation_log,
        days
    )
}
