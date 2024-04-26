use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub r: f64,
    pub i: f64,
}

impl Complex {
    pub fn len(self) -> f64 {
        ((self.r * self.r) + (self.i * self.i)).powf(0.5f64)
    }

    pub fn sin(self) -> Self {
        Self {
            r: f64::sin(self.r),
            i: f64::sin(self.i),
        }
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, c: Self) -> Self {
        Self {
            r: self.r + c.r,
            i: self.i + c.i,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, c: Self) {
        *self = *self + c;
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, c: Self) -> Self {
        Self {
            r: self.r - c.r,
            i: self.i - c.i,
        }
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, c: Self) {
        *self = *self - c;
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, c: Self) -> Self {
        Self {
            r: (self.r * c.r) - (self.i * c.i),
            i: (self.r * c.i) + (self.i * c.r),
        }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, c: Self) {
        *self = *self * c;
    }
}

