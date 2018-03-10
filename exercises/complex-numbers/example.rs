#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    re: f64,
    im: f64
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex {re, im}
    }

    pub fn exp(&self) -> Complex {
        Complex {
            re: self.re.exp() * self.im.cos(), 
            im: self.re.exp() * self.im.sin()
        }
    }

    pub fn re(&self) -> f64 {
        self.re
    }

    pub fn im(&self) -> f64 {
        self.im
    }

    pub fn abs(&self) -> f64 {
        (self.im * self.im + self.re * self.re).sqrt()
    }

    pub fn conjugate(&self) -> Complex {
        Complex::new(self.re, -self.im)
    }
}

use std::ops::{Add, Div, Mul, Sub};

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex::new((self.re * rhs.re) - (self.im * rhs.im), (self.im * rhs.re) + (self.re * rhs.im))
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Self::Output {
        Complex::new((self.re * rhs.re + self.im * rhs.im)/(rhs.re * rhs.re + rhs.im * rhs.im), (self.im * rhs.re - self.re * rhs.im)/(rhs.re * rhs.re + rhs.im * rhs.im))
    }
}