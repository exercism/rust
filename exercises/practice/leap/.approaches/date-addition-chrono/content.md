# `chrono::Date` addition

```rust
use chrono::{prelude::*, Duration};

pub fn is_leap_year(year: u64) -> bool {
    (Utc.ymd(year as i32, 2, 28) + Duration::days(1)).day() == 29
}
```

```exercism/caution
This approach may be considered a "cheat" for this exercise.
```
By adding a day to February 28th for the year, you can see if the new day is the 29th or the 1st.
If it is the 29th, then the year is a leap year.
This is done by using the [`Duration::days(1)`][day-duration] method to add a day to a [`chrono::Date`][chrono-date] `struct` and comparing it to `29` with the [`day`][day-method] method.

The documentation suggests using [NaiveDate][naive] instead of `Date`, in which case the code would be

```rust
pub fn is_leap_year_naive(year: u64) -> bool {
    (NaiveDate::from_ymd(year as i32, 2, 28) + chrono::Duration::days(1)).day() == 29
}
```

Running benchmarks gave similar results for `Date` and `NaiveDate`.

[day-duration]: https://docs.rs/chrono/latest/chrono/struct.Duration.html#method.days
[chrono-date]: https://docs.rs/chrono/latest/chrono/struct.Date.html
[day-method]: https://docs.rs/chrono/latest/chrono/struct.Date.html#method.day
[naive]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html
