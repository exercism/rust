# Instructions

You're an avid bird watcher that keeps track of how many birds have visited your garden in the last seven days.

You have six tasks, all dealing with the numbers of birds that visited your garden.

## 1. Check what the counts were last week

For comparison purposes, you always keep a copy of last week's log nearby,
which were: 0, 2, 5, 3, 7, 8 and 4. Implement the `last_week_log()` function
that returns last week's log

```rust
last_week_log();
// => [0, 2, 5, 3, 7, 8, 4]
```

## 2. Check how many birds visited today

Implement the `count_today()` function to return how many birds visited your garden today. The bird counts are ordered by day, with the first element being the count of the oldest day, and the last element being today's count.

```rust
let watch_log = [ 2, 5, 0, 7, 4, 1 ];
count_today(watch_log);
// => 1
```

## 3. Increment today's count

Implement the `log_today()` function to increment today's count:

```rust
let watch_log = [ 2, 5, 0, 7, 4, 1 ];
log_today(watch_log);
count_today(watch_log);
// => 2
```

## 4. Check if there was a day with no visiting birds

Implement the `has_day_without_birds()` method that returns `true` if there was a day at which zero birds visited the garden; otherwise, return `false`:

```rust
let watch_log = [ 2, 5, 0, 7, 4, 1 ];
has_day_without_birds(watch_log);
// => true
```

## 5. Calculate the number of visiting birds for the first x number of days

Implement the `tally_days()` function that returns the number of birds that have visited your garden from the start of the week, but limit the count to the specified number of days from the start of the week.

```rust
let watch_log = [ 2, 5, 0, 7, 4, 1 ];
tally_days(watch_log, 4);
// => 14
```

## 6. Calculate the number of busy days

Some days are busier that others. A busy day is one where five or more birds have visited your garden.
Implement the `calc_busy_days()` function to return the number of busy days:

```rust
let watch_log = [ 2, 5, 0, 7, 4, 1 ];
calc_busy_days(watch_log);
// => 2
```
