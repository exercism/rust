extern crate num;

use num::Num;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T: Num + PartialOrd + PartialEq + Copy> Triangle<T> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn valid_sides(&self) -> bool {
        let z = self.sides[0] - self.sides[0];
        (self.sides.iter().all(|&s| s > z)) &&
            (self.sides[0] + self.sides[1] >= self.sides[2]) &&
            (self.sides[1] + self.sides[2] >= self.sides[0]) &&
            (self.sides[2] + self.sides[0] >= self.sides[1])
    }

    pub fn build(sides: [T; 3]) -> Result<Self, ()> {
        let t = Triangle { sides: sides };

        if t.valid_sides() {
            Ok(t)
        } else {
            Err(())
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let a = self.sides[0];
        let b = self.sides[1];
        let c = self.sides[2];

        a == b && b == c
    }

    pub fn is_isosceles(&self) -> bool {
        let a = self.sides[0];
        let b = self.sides[1];
        let c = self.sides[2];

        (a == b && a != c) || (a == c && a != b) || (b == c && a != b)
    }

    pub fn is_scalene(&self) -> bool {
        let a = self.sides[0];
        let b = self.sides[1];
        let c = self.sides[2];

        a != b && a != c && b != c
    }
}
