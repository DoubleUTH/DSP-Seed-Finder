use std::sync::LazyLock;

use super::random::DspRandom;
use crate::data::vector3::Vector3;

const TABLE_SIZE: usize = 65536;

/// Pre-generated random table matching C# `RandomTable`.
/// `SphericNormal` generates normally-distributed 3D points (Box-Muller + rejection sampling)
/// for use in planet terrain algorithms (algo4, algo10, etc.).
pub struct RandomTable {
    spheric_normal: Box<[Vector3; TABLE_SIZE]>,
}

static RANDOM_TABLE: LazyLock<RandomTable> = LazyLock::new(|| RandomTable::generate());

/// Box-Muller transform matching C# `RandomTable.Normal(System.Random rand)`.
/// Returns a standard normally-distributed value N(0,1).
fn normal(rand: &mut DspRandom) -> f64 {
    let num = rand.next_f64();
    let a = rand.next_f64() * std::f64::consts::PI * 2.0;
    (-2.0 * (1.0 - num).ln()).sqrt() * a.sin()
}

impl RandomTable {
    /// Generates the pre-computed spheric_normal table using DspRandom with seed 1001,
    /// exactly matching C# `RandomTable.GenerateSphericNormal()`.
    fn generate() -> Self {
        let mut rand = DspRandom::new_system_random(1001);
        let mut spheric_normal = Box::new([Vector3(0.0, 0.0, 0.0); TABLE_SIZE]);

        for entry in spheric_normal.iter_mut() {
            let result = loop {
                let n1 = rand.next_f64() * 2.0 - 1.0;
                let n2 = rand.next_f64() * 2.0 - 1.0;
                let n3 = rand.next_f64() * 2.0 - 1.0;
                let n4 = normal(&mut rand);

                if n4 <= 5.0 && n4 >= -5.0 {
                    let d = n1 * n1 + n2 * n2 + n3 * n3;
                    if d <= 1.0 && d >= 1e-6 {
                        let num5 = n4 / d.sqrt();
                        break (n1 * num5, n2 * num5, n3 * num5);
                    }
                }
            };

            *entry = Vector3(result.0, result.1, result.2);
        }

        Self { spheric_normal }
    }

    /// Returns a normally-distributed 3D point scaled by `scale`, advancing the seed
    /// as an index into the pre-generated table (matching C# `RandomTable.SphericNormal`).
    ///
    /// The `seed` is incremented and masked to 16 bits (0..65535) on each call.
    pub fn spheric_normal(seed: &mut i32, scale: f64) -> Vector3 {
        *seed = seed.wrapping_add(1) & 0xFFFF;
        let v = RANDOM_TABLE.spheric_normal[*seed as usize];
        v * scale
    }
}
