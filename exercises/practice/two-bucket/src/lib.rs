use std::cmp::max;

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

fn get_gcd(a: &u8, b: &u8) -> u8 {
    let mut a = *a;
    let mut b = *b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {

    let mut res = BucketStats{
        moves:0,
        goal_bucket: Bucket::One,
        other_bucket: 2
    };

    // Goal larger than both buckets: not possible
    if goal > max(capacity_1, capacity_2) {
        return None;
    }

    // It is only possible to get multiples of the greatest common factor of both numbers.
    let gcd = get_gcd(&capacity_1, &capacity_2); 
    if goal % gcd != 0 {
        return None;
    }

    // To be agnostic of bucket one or two.
    let (size_b1, size_b2) = match *start_bucket {
        Bucket::One => (capacity_1, capacity_2),
        Bucket::Two => (capacity_2, capacity_1)
    };

    // Initial step, first half
    let mut in_b1: u8 = match *start_bucket {
        Bucket::One => capacity_1,
        Bucket::Two => capacity_2
    };
    let mut in_b2: u8 = 0;
    res.moves += 1;

    // Complete second half of first step
    if in_b1 != goal {
        if size_b2 == goal {
            in_b2 = goal;
            res.moves += 1; 
        }
        else {
            let to_xfer = in_b1.min(size_b2 - in_b2);
            in_b2 += to_xfer;
            in_b1 -= to_xfer;
            res.moves += 1; 
        }
    }

    // main loop
    while in_b1 != goal && in_b2 != goal {

        // first half of a step
        if in_b2 == size_b2 {
            in_b2 = 0;
        }
        else {
            in_b1 = size_b1;
        }
        res.moves += 1;
        if in_b1 == goal || in_b2 == goal {
            break;
        }
        
        // second half of a step
        let to_xfer = in_b1.min(size_b2 - in_b2);
        in_b2 += to_xfer;
        in_b1 -= to_xfer;
        res.moves += 1;
        if in_b1 == goal || in_b2 == goal {
            break;
        }

    }

    // Format output
    // Handle goal_bucket
    match *start_bucket {
        Bucket::One => {
            if in_b1 == goal {
                res.goal_bucket = Bucket::One;
            }
            else {
                res.goal_bucket = Bucket::Two;
            }
        },
        Bucket::Two => {
            if in_b1 == goal {
                res.goal_bucket = Bucket::Two;
            }
            else {
                res.goal_bucket = Bucket::One;
            }
        }
    }

    // Handle other_bucket
    if in_b1 == goal {
        res.other_bucket = in_b2;
    }
    else {
        res.other_bucket = in_b1;
    }

    Some(res)
}
