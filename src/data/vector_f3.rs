use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::vector3::Vector3;

/// f32-based 3D vector, used for birth point generation
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VectorF3(pub f32, pub f32, pub f32);

impl VectorF3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn up() -> Self {
        Self(0.0, 1.0, 0.0)
    }

    pub fn forward() -> Self {
        Self(0.0, 0.0, 1.0)
    }

    pub fn back() -> Self {
        Self(0.0, 0.0, -1.0)
    }

    pub fn right() -> Self {
        Self(1.0, 0.0, 0.0)
    }

    pub fn left() -> Self {
        Self(-1.0, 0.0, 0.0)
    }

    pub fn down() -> Self {
        Self(0.0, -1.0, 0.0)
    }

    pub fn magnitude_sq(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
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

    pub fn distance_sq_from(&self, pt: &Self) -> f32 {
        let dx = pt.0 - self.0;
        let dy = pt.1 - self.1;
        let dz = pt.2 - self.2;
        dx * dx + dy * dy + dz * dz
    }

    pub fn cross(a: &Self, b: &Self) -> Self {
        Self(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0,
        )
    }

    pub fn dot(a: &Self, b: &Self) -> f32 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    /// Convert to the f64-based Vector3 (for use with query_height)
    pub fn to_f64_vec3(&self) -> Vector3 {
        Vector3(self.0 as f64, self.1 as f64, self.2 as f64)
    }

    pub fn slerp(lhs: &VectorF3, rhs: &VectorF3, percent: f32) -> VectorF3 {
        let dot = VectorF3::dot(lhs, rhs).clamp(-1.0, 1.0);
        let theta = dot.acos() * percent;
        let mut relative_vec = rhs - &(lhs * dot);
        relative_vec.normalize();
        &(lhs * theta.cos()) + &(&relative_vec * theta.sin())
    }
}

impl Add for VectorF3 {
    type Output = VectorF3;

    fn add(self, rhs: VectorF3) -> VectorF3 {
        VectorF3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<&VectorF3> for VectorF3 {
    type Output = VectorF3;

    fn add(self, rhs: &VectorF3) -> VectorF3 {
        VectorF3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<&VectorF3> for &VectorF3 {
    type Output = VectorF3;

    fn add(self, rhs: &VectorF3) -> VectorF3 {
        VectorF3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for VectorF3 {
    fn add_assign(&mut self, rhs: VectorF3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl AddAssign<&VectorF3> for VectorF3 {
    fn add_assign(&mut self, rhs: &VectorF3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for VectorF3 {
    type Output = VectorF3;

    fn sub(self, rhs: VectorF3) -> VectorF3 {
        VectorF3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<&VectorF3> for VectorF3 {
    type Output = VectorF3;

    fn sub(self, rhs: &VectorF3) -> VectorF3 {
        VectorF3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<&VectorF3> for &VectorF3 {
    type Output = VectorF3;

    fn sub(self, rhs: &VectorF3) -> VectorF3 {
        VectorF3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for VectorF3 {
    fn sub_assign(&mut self, rhs: VectorF3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl SubAssign<&VectorF3> for VectorF3 {
    fn sub_assign(&mut self, rhs: &VectorF3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f32> for VectorF3 {
    type Output = VectorF3;

    fn mul(self, rhs: f32) -> VectorF3 {
        VectorF3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<f32> for &VectorF3 {
    type Output = VectorF3;

    fn mul(self, rhs: f32) -> VectorF3 {
        VectorF3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f32> for VectorF3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f32> for VectorF3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Div<f32> for VectorF3 {
    type Output = VectorF3;

    fn div(self, rhs: f32) -> VectorF3 {
        VectorF3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Div<f32> for &VectorF3 {
    type Output = VectorF3;

    fn div(self, rhs: f32) -> VectorF3 {
        VectorF3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Neg for VectorF3 {
    type Output = VectorF3;

    fn neg(self) -> VectorF3 {
        VectorF3(-self.0, -self.1, -self.2)
    }
}

impl std::fmt::Display for VectorF3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}
