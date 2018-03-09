#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    // Add in the fields
}

impl Complex {
    pub fn new(_re: f64, _im: f64) -> Complex {
        unimplemented!()
    }

    pub fn exp(&self) -> Complex {
        unimplemented!()
    }

    // Return the real part
    pub fn re(&self) -> f64 {
        unimplemented!()
    }

    // Return the imaginary part
    pub fn im(&self) -> f64 {
        unimplemented!()
    }

    // Return the absolute value
    pub fn abs(&self) -> f64 {
        unimplemented!()
    }
    
    // Return the conjugate
    pub fn conjugate(&self) -> Complex {
        unimplemented!()
    }
}

// Implement `std::ops` for `Complex`

use std::ops::{Add, Div, Mul, Sub};

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, _rhs: Complex) -> Self::Output {
        unimplemented!()
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, _rhs: Complex) -> Self::Output {
        unimplemented!()
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, _rhs: Complex) -> Self::Output {
        unimplemented!()
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, _rhs: Complex) -> Self::Output {
        unimplemented!()
    }
}