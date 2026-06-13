use super::quaternion::Quaternion;
use super::vector_f3::VectorF3;

/// Port of Unity's `Pose(Vector3 position, Quaternion rotation)`.
/// Uses VectorF3 for position (f32-based, matching VectorLF3 → VectorF3 mapping).
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pose {
    pub position: VectorF3,
    pub rotation: Quaternion,
}

impl Pose {
    pub fn new(position: VectorF3, rotation: Quaternion) -> Self {
        Self { position, rotation }
    }
}
