# Instructions

You're an avid bird watcher who keeps track of the number of birds that visit your garden.

You have data stored in an *observation log*, from which you like to gather a some information.

## 1. Determine the total number of birds you've recorded so for

Find out how many birds there are in total, since you started logging. Implement function `bird_count` that accepts a vector containing the bird count per day. This function should return the total number of birds present in the vector.

```rust
let birds_per_day = vec![9, 0, 8, 4, 5, 1, 3];
total_bird_count(birds_per_day)
// Returns: 30
```

## 2. Calculate the number of birds in a specific week

You want insight into the number of birds that visited your garden in a specific week.

Implement function `birds_in_week` that accepts a vector containing the bird count per day and a week number. This function will calculate the total number of birds that have visited in the specified week.

You can assume weeks are always tracked completely.

```rust
let birs_per_day = vec![2, 5, 0, 7, 4, 1, 3, 0, 2, 5, 0, 1, 3, 1];
let week = 2;
birds_in_week(birds_per_day, week)
// Returns: 12
```

## 3. Fix a counting mistake

You realized that while you were tracking birds, there was one bird hiding in a far corner of the garden.

Your bird watcher intuition tells you that this bird was in your garden on day you started logging.

You also figured out that this bird spent every second day in your garden.

Given this new information, write a function `fix_bird_count` that takes a vector of bird count per day as an argument and returns a modified vector which contains the correct information.

```rust
let birds_per_day = vec![2, 5, 0, 7, 4, 1];
fix_bird_count(birds_per_day)
// Returns: [3 5 1 7 5 1]
```

## 4. Determine the number of birds that visited today

Your observation log contains bird count ordered by day. This means that the first count is for the oldest day, and the last count is for the most recent day (today)

Implement function `birds_today` which returns the number of birds that visited your garden today. If no birds visited the garden today, return `-1`.

```rust
let birds_per_day = vec![2, 5, 0, 7, 4, 8];
birds_today(birds_per_day)
// Returns: 8
```

## 5. Determine day with no visiting birds

Implement function `day_without_birds` which accepts a vector of birds count per day.

This function should return `true` if there's a day which has in which no bird visited your garden, and otherwise `false`

```rust
let birds_per_day = vec![2, 5, 0, 7, 4, 1];
day_without_birds(birds_per_day)
// Returns: true
```

## 6. Calculate the number of busy days

The frequency of visiting birds is likely to vary. A busy day is one in which five or more birds visit your garden.

Implement function `busy_days_number` which takes a vector containing the bird count per day and returns the number of busy days.

```rust
let birds_per_day = vec![2, 5, 0, 7, 4, 1];
busy_days_number(birds_per_day)
// Returns: 2
```

## 7. Calculate the number of birds in the first N days

Implement function `birds_in_first_days` which takes two parameters - a vector containing the bird count per day, and *n* for count up to day.

This function returns the number of birds which visited your garden in the first *n* days.

```rust
let birds_per_day = vec![2, 5, 0, 7, 4, 1];
let n = 4;
birds_in_first_days(birds_per_day, n)
// Returns: 14
```
