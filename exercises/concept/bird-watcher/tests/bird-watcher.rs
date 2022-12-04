#[test]
fn test_bird_count() {
    let observations = [
        (vec![9, 0, 8, 4, 5, 1, 3], 30, "7 day bird visit log"),
        (vec![2], 2, "1 day bird visit log"),
        (
            vec![17, 15, 20, 12, 14, 21, 23, 20, 21, 19, 16, 22, 26, 11],
            257,
            "14 day bird visit log",
        ),
    ];

    for (log_period, total, description) in observations {
        assert_eq!(
            bird_watcher::bird_count(log_period),
            total,
            "{}",
            description
        )
    }
}

#[test]
#[ignore]
fn test_birds_in_week() {
    let observations = [
        (
            vec![3, 0, 5, 1, 0, 4, 1, 0, 3, 4, 3, 0, 8, 0],
            1,
            14,
            "total birds in week 1 of 2",
        ),
        (
            vec![
                4, 7, 3, 2, 1, 1, 2, 0, 2, 3, 2, 7, 1, 3, 0, 6, 5, 3, 7, 2, 3,
            ],
            2,
            18,
            "total birds in week 2 of 3",
        ),
        (
            vec![3, 0, 3, 3, 2, 1, 0],
            1,
            12,
            "total birds in week 1 of 1",
        ),
    ];

    for (log_period, week, total, description) in observations {
        assert_eq!(
            bird_watcher::birds_in_week(log_period, week),
            total,
            "{}",
            description
        )
    }
}

#[test]
#[ignore]
fn test_fix_bird_count() {
    let observations = [
        (
            vec![3, 0, 5, 1, 0, 4, 1, 0, 3, 4, 3, 0],
            vec![4, 0, 6, 1, 1, 4, 2, 0, 4, 4, 4, 0],
            "update missing bird on odd days in 12 day period",
        ),
        (
            vec![4, 2],
            vec![5, 2],
            "update missing bird on odd day in 2 day period",
        ),
        (
            vec![2, 8, 4, 1, 3],
            vec![3, 8, 5, 1, 4],
            "update missing bird on odd days in 5 day period",
        ),
    ];

    for (initial_observation, final_observation, description) in observations {
        assert_eq!(
            bird_watcher::fix_bird_count(initial_observation),
            final_observation,
            "{}",
            description
        )
    }
}

#[test]
#[ignore]
fn test_birds_today() {
    let observations = [
        (
            vec![2, 5, 0, 7, 4, 1],
            1,
            "birds are ordered by day - last element is today's count, which is 1",
        ),
        (
            vec![0, 2, 5, 3, 7, 8, 4],
            4,
            "birds are ordered by day - today's count is 4",
        ),
        (vec![0], -1, "no birds visited the garden today"),
    ];

    for (log_period, count, description) in observations {
        assert_eq!(
            bird_watcher::birds_today(log_period),
            count,
            "{}",
            description
        )
    }
}

#[test]
#[ignore]
fn test_day_without_birds() {
    let observations = [
        (
            vec![2, 5, 0, 7, 4, 1],
            true,
            "no birds visited the garden on the 3rd day",
        ),
        (
            vec![1, 4, 3, 2, 2, 5, 2],
            false,
            "atleast one bird visited the garden everyday",
        ),
    ];

    for (log_period, expected, description) in observations {
        assert_eq!(
            bird_watcher::day_without_birds(log_period),
            expected,
            "{}",
            description
        )
    }
}

#[test]
#[ignore]
fn test_busy_days_number() {
    let observations = [
        (
            vec![2, 5, 0, 7, 4, 1],
            2,
            "5 and 7 birds visit the garden on separate days",
        ),
        (vec![1, 2, 3, 4], 0, "fewer than 5 birds visit"),
        (
            vec![0, 6, 1, 3, 5, 8, 10, 11],
            5,
            "more than 5 birds visit on 4 occasions",
        ),
    ];

    for (log_period, expected, description) in observations {
        assert_eq!(
            bird_watcher::busy_days_number(log_period),
            expected,
            "{}",
            description
        )
    }
}

#[test]
#[ignore]
fn test_birds_in_first_days() {
    let observations = [
        (
            vec![2, 5, 0, 7, 4, 1],
            4,
            14,
            "14 birds vosoted in the first 4 days",
        ),
        (
            vec![3, 1, 5, 8, 2],
            1,
            3,
            "3 birds visited on the first day",
        ),
        (
            vec![6, 8, 3, 9],
            10,
            26,
            "since the value of days is greater than the observation data, return total",
        ),
    ];

    for (log_period, days, expected, description) in observations {
        assert_eq!(
            bird_watcher::birds_in_first_days(log_period, days),
            expected,
            "{}",
            description
        )
    }
}
