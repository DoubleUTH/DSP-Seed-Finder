use super::vector_f3::VectorF3;
use std::ops::Mul;

/// Port of Unity's Quaternion (x, y, z, w) using f32.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// Port of Unity's `Quaternion.AngleAxis(angle_degrees, axis)`.
    /// `angle_degrees` is in degrees (same as Unity's AngleAxis).
    pub fn angle_axis(angle_degrees: f32, axis: &VectorF3) -> Self {
        let half_rad = (angle_degrees * std::f32::consts::PI / 180.0) * 0.5;
        let s = half_rad.sin();
        let axis_norm = axis.normalized();
        Self {
            x: axis_norm.0 * s,
            y: axis_norm.1 * s,
            z: axis_norm.2 * s,
            w: half_rad.cos(),
        }
    }

    /// Port of `Maths.QInvRotateLF(Quaternion, VectorLF3)` — inverse rotates a VectorF3.
    /// Since the user specified VectorLF3 maps to VectorF3 in Rust, this uses f32.
    pub fn q_inv_rotate_lf(&self, v: &VectorF3) -> VectorF3 {
        let vx = v.0 * 2.0;
        let vy = v.1 * 2.0;
        let vz = v.2 * 2.0;
        let num1 = self.w * self.w - 0.5;
        let num2 = self.x * vx + self.y * vy + self.z * vz;
        VectorF3(
            vx * num1 - (self.y * vz - self.z * vy) * self.w + self.x * num2,
            vy * num1 - (self.z * vx - self.x * vz) * self.w + self.y * num2,
            vz * num1 - (self.x * vy - self.y * vx) * self.w + self.z * num2,
        )
    }

    /// Port of Unity's `Quaternion.FromToRotation(fromDirection, toDirection)`.
    ///
    /// Creates a rotation that rotates `from` to align with `to`.
    pub fn from_to_rotation(from: &VectorF3, to: &VectorF3) -> Self {
        let from_mag_sq = from.magnitude_sq();
        let to_mag_sq = to.magnitude_sq();
        if from_mag_sq < 1e-10 || to_mag_sq < 1e-10 {
            return Self::identity();
        }

        let from_norm = from.normalized();
        let to_norm = to.normalized();

        let dot = VectorF3::dot(&from_norm, &to_norm);

        // Vectors are nearly opposite — 180° rotation around an orthogonal axis
        if dot < -0.999999 {
            // Find an orthogonal axis to rotate around
            let abs_x = from_norm.0.abs();
            let abs_y = from_norm.1.abs();
            let abs_z = from_norm.2.abs();

            let axis = if abs_x < abs_y && abs_x < abs_z {
                VectorF3::cross(&from_norm, &VectorF3(1.0, 0.0, 0.0))
            } else if abs_y < abs_z {
                VectorF3::cross(&from_norm, &VectorF3(0.0, 1.0, 0.0))
            } else {
                VectorF3::cross(&from_norm, &VectorF3(0.0, 0.0, 1.0))
            };

            let axis_norm = axis.normalized();
            Self {
                x: axis_norm.0,
                y: axis_norm.1,
                z: axis_norm.2,
                w: 0.0,
            }
        } else {
            // Standard case: rotation axis = cross(from, to), angle = acos(dot)
            let cross = VectorF3::cross(&from_norm, &to_norm);
            let s = ((1.0 + dot) * 2.0).sqrt();
            let inv_s = 1.0 / s;
            let q = Self {
                x: cross.0 * inv_s,
                y: cross.1 * inv_s,
                z: cross.2 * inv_s,
                w: s * 0.5,
            };
            // Normalize
            let mag = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
            Self {
                x: q.x / mag,
                y: q.y / mag,
                z: q.z / mag,
                w: q.w / mag,
            }
        }
    }

    /// Port of `Maths.QRotateLF(Quaternion, VectorLF3)` — rotates a VectorF3.
    pub fn q_rotate_lf(&self, v: &VectorF3) -> VectorF3 {
        let vx = v.0 * 2.0;
        let vy = v.1 * 2.0;
        let vz = v.2 * 2.0;
        let num1 = self.w * self.w - 0.5;
        let num2 = self.x * vx + self.y * vy + self.z * vz;
        VectorF3(
            vx * num1 + (self.y * vz - self.z * vy) * self.w + self.x * num2,
            vy * num1 + (self.z * vx - self.x * vz) * self.w + self.y * num2,
            vz * num1 + (self.x * vy - self.y * vx) * self.w + self.z * num2,
        )
    }
}

/// Port of Unity's `Quaternion * Vector3` — rotates a VectorF3 by a Quaternion.
///
/// Uses the full 3×3 rotation matrix derived from the quaternion, matching Unity's
/// internal implementation (which is Fused in IL2CPP builds).
impl Mul<&VectorF3> for &Quaternion {
    type Output = VectorF3;

    fn mul(self, point: &VectorF3) -> VectorF3 {
        let num1 = self.x * 2.0;
        let num2 = self.y * 2.0;
        let num3 = self.z * 2.0;
        let num4 = self.x * num1;
        let num5 = self.y * num2;
        let num6 = self.z * num3;
        let num7 = self.x * num2;
        let num8 = self.x * num3;
        let num9 = self.y * num3;
        let num10 = self.w * num1;
        let num11 = self.w * num2;
        let num12 = self.w * num3;
        VectorF3(
            (1.0 - (num5 + num6)) * point.0 + (num7 - num12) * point.1 + (num8 + num11) * point.2,
            (num7 + num12) * point.0 + (1.0 - (num4 + num6)) * point.1 + (num9 - num10) * point.2,
            (num8 - num11) * point.0 + (num9 + num10) * point.1 + (1.0 - (num4 + num5)) * point.2,
        )
    }
}

/// Port of `Maths.QMultiply` for quaternion composition (equivalent to Unity's `q1 * q2`).
impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x * rhs.w + self.y * rhs.z - self.z * rhs.y + self.w * rhs.x,
            y: -self.x * rhs.z + self.y * rhs.w + self.z * rhs.x + self.w * rhs.y,
            z: self.x * rhs.y - self.y * rhs.x + self.z * rhs.w + self.w * rhs.z,
            w: -self.x * rhs.x - self.y * rhs.y - self.z * rhs.z + self.w * rhs.w,
        }
    }
}

impl Mul<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.x * rhs.w + self.y * rhs.z - self.z * rhs.y + self.w * rhs.x,
            y: -self.x * rhs.z + self.y * rhs.w + self.z * rhs.x + self.w * rhs.y,
            z: self.x * rhs.y - self.y * rhs.x + self.z * rhs.w + self.w * rhs.z,
            w: -self.x * rhs.x - self.y * rhs.y - self.z * rhs.z + self.w * rhs.w,
        }
    }
}

impl Mul<Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x * rhs.w + self.y * rhs.z - self.z * rhs.y + self.w * rhs.x,
            y: -self.x * rhs.z + self.y * rhs.w + self.z * rhs.x + self.w * rhs.y,
            z: self.x * rhs.y - self.y * rhs.x + self.z * rhs.w + self.w * rhs.z,
            w: -self.x * rhs.x - self.y * rhs.y - self.z * rhs.z + self.w * rhs.w,
        }
    }
}

impl Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.x * rhs.w + self.y * rhs.z - self.z * rhs.y + self.w * rhs.x,
            y: -self.x * rhs.z + self.y * rhs.w + self.z * rhs.x + self.w * rhs.y,
            z: self.x * rhs.y - self.y * rhs.x + self.z * rhs.w + self.w * rhs.z,
            w: -self.x * rhs.x - self.y * rhs.y - self.z * rhs.z + self.w * rhs.w,
        }
    }
}
