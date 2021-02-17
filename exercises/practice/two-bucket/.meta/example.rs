use std::collections::{HashSet, VecDeque};

// We can turn this problem into a simple graph searching problem. Each move represents an
// edge in our graph; the buckets' fill states form the graph's vertices. For example, say bucket
// one holds up to 7 liters, and bucket two holds up to 11 liters, and at the current step, bucket
// one has 7 liters and bucket two has 8 liters: (7, 8). By emptying the first bucket, we form an
// edge (7, 8) -> (0, 8). Similarly, by pouring the first bucket into the second, we form an edge
// (7, 8) -> (4, 11).
//
// Since we want to minimize the number of moves, we search the graph breadth-first, starting with
// the configuration provided as the problem input. Note that, to avoid cheating, we mark both
// possible starting configurations as visited; otherwise, our search might just empty the initial
// bucket and fill the other one.

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let state = match *start_bucket {
        Bucket::One => (capacity_1, 0),
        Bucket::Two => (0, capacity_2),
    };

    let mut next_search = VecDeque::new();
    let mut visited = HashSet::new();
    let mut moves = 1;

    next_search.push_front(state);

    // "Visit" both starting states. This will ensure that we don't cheat, i.e.
    // empty our starting bucket completely and fill the other bucket.
    visited.insert((capacity_1, 0));
    visited.insert((0, capacity_2));

    while !next_search.is_empty() {
        let mut current_search = next_search;
        next_search = VecDeque::new();

        for state in current_search.drain(0..) {
            let (bucket_1, bucket_2) = state;

            if bucket_1 == goal {
                return Some(BucketStats {
                    moves,
                    goal_bucket: Bucket::One,
                    other_bucket: bucket_2,
                });
            } else if bucket_2 == goal {
                return Some(BucketStats {
                    moves,
                    goal_bucket: Bucket::Two,
                    other_bucket: bucket_1,
                });
            }

            // Empty the first bucket
            let empty_1 = (0, bucket_2);
            if !visited.contains(&empty_1) {
                next_search.push_front(empty_1);
                visited.insert(empty_1);
            }

            // Empty the second bucket
            let empty_2 = (bucket_1, 0);
            if !visited.contains(&empty_2) {
                next_search.push_front(empty_2);
                visited.insert(empty_2);
            }

            // Fill the first bucket
            let fill_1 = (capacity_1, bucket_2);
            if !visited.contains(&fill_1) {
                next_search.push_front(fill_1);
                visited.insert(fill_1);
            }

            // Fill the second bucket
            let fill_2 = (bucket_1, capacity_2);
            if !visited.contains(&fill_2) {
                next_search.push_front(fill_2);
                visited.insert(fill_2);
            }

            // Pour the first bucket into the second bucket
            let pour_1_into_2 = if bucket_1 + bucket_2 <= capacity_1 {
                (bucket_1 + bucket_2, 0)
            } else {
                (capacity_1, bucket_1 + bucket_2 - capacity_1)
            };
            if !visited.contains(&pour_1_into_2) {
                next_search.push_front(pour_1_into_2);
                visited.insert(pour_1_into_2);
            }

            // Pour the second bucket into the first bucket
            let pour_2_into_1 = if bucket_1 + bucket_2 <= capacity_2 {
                (0, bucket_1 + bucket_2)
            } else {
                (bucket_1 + bucket_2 - capacity_2, capacity_2)
            };
            if !visited.contains(&pour_2_into_1) {
                next_search.push_front(pour_2_into_1);
                visited.insert(pour_2_into_1);
            }
        }

        moves += 1;
    }

    // We ran out of states to search but still didn't reach the goal.
    None
}
