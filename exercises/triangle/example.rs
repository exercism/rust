use std::collections::BTreeSet;
use std::iter::FromIterator;

pub struct Triangle {
    sides: [u16; 3],
}

impl Triangle {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn valid_sides(&self) -> bool {
        (self.sides.iter().all(|&s| s > 0)) &&
            (self.sides[0] + self.sides[1] >= self.sides[2]) &&
            (self.sides[1] + self.sides[2] >= self.sides[0]) &&
            (self.sides[2] + self.sides[0] >= self.sides[1])
    }

    pub fn build(sides: [u16; 3]) -> Result<Self, ()> {
        let t = Triangle { sides: sides };

        if t.valid_sides() {
            Ok(t)
        } else {
            Err(())
        }
    }

    pub fn is_equilateral(&self) -> bool {
        BTreeSet::from_iter(self.sides.iter()).len() == 1
    }

    pub fn is_isosceles(&self) -> bool {
        BTreeSet::from_iter(self.sides.iter()).len() == 2
    }

    pub fn is_scalene(&self) -> bool {
        BTreeSet::from_iter(self.sides.iter()).len() == 3
    }
}
