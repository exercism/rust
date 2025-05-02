#[test]
pub fn last_week() {
    assert_eq!([0, 2, 5, 3, 7, 8, 4], bird_watcher::last_week_log());
}

#[test]
#[ignore]
pub fn today_for_disappointing_day() {
    let bird_watch_log = [0, 0, 1, 0, 0, 1, 0];
    assert_eq!(0, bird_watcher::count_today(bird_watch_log));
}

#[test]
#[ignore]
pub fn today_for_busy_day() {
    let bird_watch_log = [8, 8, 9, 5, 4, 7, 10];
    assert_eq!(10, bird_watcher::count_today(bird_watch_log));
}

#[test]
#[ignore]
pub fn increment_todays_count_with_no_previous_visits() {
    let mut bird_watch_log = [0, 0, 0, 4, 2, 3, 0];
    bird_watcher::log_today(&mut bird_watch_log);
    assert_eq!(1, bird_watcher::count_today(bird_watch_log));
}

#[test]
#[ignore]
pub fn increment_todays_count_with_multiple_previous_visits() {
    let mut bird_watch_log = [8, 8, 9, 2, 1, 6, 4];
    bird_watcher::log_today(&mut bird_watch_log);
    assert_eq!(5, bird_watcher::count_today(bird_watch_log));
}

#[test]
#[ignore]
pub fn has_day_without_birds_with_day_without_birds() {
    let bird_watch_log = [5, 5, 4, 0, 7, 6, 7];

    assert!(bird_watcher::has_day_without_birds(bird_watch_log));
}

#[test]
#[ignore]
pub fn has_day_without_birds_with_no_day_without_birds() {
    let bird_watch_log = [4, 5, 9, 10, 9, 4, 3];

    assert!(!bird_watcher::has_day_without_birds(bird_watch_log));
}

#[test]
#[ignore]
pub fn count_for_first_three_days_of_disappointing_week() {
    let bird_watch_log = [0, 0, 1, 0, 0, 1, 0];

    assert_eq!(1, bird_watcher::tally_days(bird_watch_log, 3));
}

#[test]
#[ignore]
pub fn count_for_first_six_days_of_busy_week() {
    let bird_watch_log = [5, 9, 12, 6, 8, 8, 17];

    assert_eq!(48, bird_watcher::tally_days(bird_watch_log, 6));
}

#[test]
#[ignore]
pub fn busy_days_for_disappointing_week() {
    let bird_watch_log = [1, 1, 1, 0, 0, 0, 0];

    assert_eq!(0, bird_watcher::calc_busy_days(bird_watch_log));
}

#[test]
#[ignore]
pub fn busy_days_for_busy_week() {
    let bird_watch_log = [4, 9, 5, 7, 8, 8, 2];

    assert_eq!(5, bird_watcher::calc_busy_days(bird_watch_log));
}
