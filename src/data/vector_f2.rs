use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// f32-based 2D vector, port of Unity's Vector2.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VectorF2(pub f32, pub f32);

impl VectorF2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    pub fn zero() -> Self {
        Self(0.0, 0.0)
    }

    pub fn up() -> Self {
        Self(0.0, 1.0)
    }

    pub fn down() -> Self {
        Self(0.0, -1.0)
    }

    pub fn right() -> Self {
        Self(1.0, 0.0)
    }

    pub fn left() -> Self {
        Self(-1.0, 0.0)
    }

    pub fn magnitude_sq(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_sq().sqrt()
    }

    pub fn normalize(&mut self) -> &mut Self {
        let mag = self.magnitude();
        if mag > 1e-10 {
            *self /= mag;
        } else {
            *self = Self::zero();
        }
        self
    }

    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag > 1e-10 {
            *self / mag
        } else {
            Self::zero()
        }
    }

    pub fn dot(a: &Self, b: &Self) -> f32 {
        a.0 * b.0 + a.1 * b.1
    }

    pub fn distance_sq_from(&self, pt: &Self) -> f32 {
        let dx = pt.0 - self.0;
        let dy = pt.1 - self.1;
        dx * dx + dy * dy
    }
}

impl Add for VectorF2 {
    type Output = VectorF2;

    fn add(self, rhs: VectorF2) -> VectorF2 {
        VectorF2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<&VectorF2> for VectorF2 {
    type Output = VectorF2;

    fn add(self, rhs: &VectorF2) -> VectorF2 {
        VectorF2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<&VectorF2> for &VectorF2 {
    type Output = VectorF2;

    fn add(self, rhs: &VectorF2) -> VectorF2 {
        VectorF2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for VectorF2 {
    fn add_assign(&mut self, rhs: VectorF2) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl AddAssign<&VectorF2> for VectorF2 {
    fn add_assign(&mut self, rhs: &VectorF2) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for VectorF2 {
    type Output = VectorF2;

    fn sub(self, rhs: VectorF2) -> VectorF2 {
        VectorF2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<&VectorF2> for VectorF2 {
    type Output = VectorF2;

    fn sub(self, rhs: &VectorF2) -> VectorF2 {
        VectorF2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<&VectorF2> for &VectorF2 {
    type Output = VectorF2;

    fn sub(self, rhs: &VectorF2) -> VectorF2 {
        VectorF2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for VectorF2 {
    fn sub_assign(&mut self, rhs: VectorF2) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl SubAssign<&VectorF2> for VectorF2 {
    fn sub_assign(&mut self, rhs: &VectorF2) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<f32> for VectorF2 {
    type Output = VectorF2;

    fn mul(self, rhs: f32) -> VectorF2 {
        VectorF2(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<f32> for &VectorF2 {
    type Output = VectorF2;

    fn mul(self, rhs: f32) -> VectorF2 {
        VectorF2(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign<f32> for VectorF2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl DivAssign<f32> for VectorF2 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl Div<f32> for VectorF2 {
    type Output = VectorF2;

    fn div(self, rhs: f32) -> VectorF2 {
        VectorF2(self.0 / rhs, self.1 / rhs)
    }
}

impl Div<f32> for &VectorF2 {
    type Output = VectorF2;

    fn div(self, rhs: f32) -> VectorF2 {
        VectorF2(self.0 / rhs, self.1 / rhs)
    }
}

impl Neg for VectorF2 {
    type Output = VectorF2;

    fn neg(self) -> VectorF2 {
        VectorF2(-self.0, -self.1)
    }
}

impl std::fmt::Display for VectorF2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
