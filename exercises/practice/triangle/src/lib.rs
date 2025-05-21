use num_traits::Zero;
use std::fmt::Debug;
// use std::ops::Add;

#[derive(Debug)]
pub struct Triangle<T> {
    a: T,
    b: T,
    c: T
}

impl<T: PartialOrd + Zero + Debug + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        // All sides should have positive length
        if sides.iter().any(|side| *side <= T::zero()) {
            return None;
        }

        // A triangle is defined as : the sum of any
        // two sides length is greater than the remaining side
        let [a, b, c] = sides;
        if a + b < c {
            return None;
        }
        if b + c < a {
            return None;
        }
        if a + c < b {
            return None;
        }

        Some(Self {
            a,
            b,
            c,
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        if self.is_equilateral()  {
            return false;
        }
        if self.is_isosceles() {
            return false;
        }
        true
    }

    pub fn is_isosceles(&self) -> bool {
        println!("{:#?}", self);
        if self.a == self.b || self.a == self.c || self.b == self.c {
            println!("Bla");
            true
        }
        else {
            false
        }  
    }
}


// This is another option that does not need any external crate. 
// But using ::default() because it just happens to return 0 
// for each type, is kind of a cheat. 

// impl<T: PartialOrd + Default + Add + Debug + Copy> Triangle<T> {
//     pub fn build(sides: [T; 3]) -> Option<Triangle<T>> 
//         where <T as Add>::Output: PartialOrd<T>
//     {
//         // All sides should have positive length
//         if sides.iter().any(|side| *side <= T::default()) {
//             return None;
//         }

//         // A triangle is defined as : the sum of any
//         // two sides length is greater than the remaining side
//         let [a, b, c] = sides;
//         if a + b < c {
//             return None;
//         }
//         if b + c < a {
//             return None;
//         }
//         if a + c < b {
//             return None;
//         }

//         Some(Self {
//             a,
//             b,
//             c,
//         })
//     }

//     pub fn is_equilateral(&self) -> bool {
//         self.a == self.b && self.b == self.c
//     }

//     pub fn is_scalene(&self) -> bool {
//         if self.is_equilateral()  {
//             return false;
//         }
//         if self.is_isosceles() {
//             return false;
//         }
//         true
//     }

//     pub fn is_isosceles(&self) -> bool {
//         println!("{:#?}", self);
//         if self.a == self.b || self.a == self.c || self.b == self.c {
//             println!("Bla");
//             true
//         }
//         else {
//             false
//         }  
//     }
// }


