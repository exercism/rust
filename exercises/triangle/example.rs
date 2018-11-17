use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + PartialEq + PartialOrd + Add<Output = T> + Default,
{
    fn valid_sides(&self) -> bool {
        (self.sides.iter().all(|&s| s > T::default()))
            && (self.sides[0] + self.sides[1] >= self.sides[2])
            && (self.sides[1] + self.sides[2] >= self.sides[0])
            && (self.sides[2] + self.sides[0] >= self.sides[1])
    }

    fn count_distinct_pairs(&self) -> usize {
        [(0, 1), (0, 2), (1, 2)]
            .iter()
            .map(|&(a, b)| if self.sides[a] != self.sides[b] { 1 } else { 0 })
            .sum()
    }

    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let t = Triangle { sides };

        if t.valid_sides() {
            Some(t)
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.count_distinct_pairs() == 0
    }

    pub fn is_isosceles(&self) -> bool {
        self.count_distinct_pairs() == 2
    }

    pub fn is_scalene(&self) -> bool {
        self.count_distinct_pairs() == 3
    }
}
