# `time::Date` addition

```rust
use time::{Date, Duration, Month};

pub fn is_leap_year(year: u64) -> bool {
    (Date::from_calendar_date(year as i32, Month::February, 28).unwrap() + Duration::DAY).day()
        == 29
}
```

```exercism/caution
This approach may be considered a "cheat" for this exercise.
```
By adding a day to February 28th for the year, you can see if the new day is the 29th or the 1st.
If it is the 29th, then the year is a leap year.
This is done by adding a [`Duration::DAY`][day-duration] to a [`time::Date`][time-date] `struct` and comparing it to `29` with the [`day`][day-method] method.

[day-duration]: https://docs.rs/time/latest/time/struct.Duration.html#associatedconstant.DAY
[time-date]: https://docs.rs/time/latest/time/struct.Date.html
[day-method]: https://docs.rs/time/latest/time/struct.Date.html#method.day
