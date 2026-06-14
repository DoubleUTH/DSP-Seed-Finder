use super::random::DspRandom;
use super::vector_f3::VectorF3;
use crate::data::planet_raw_data::PlanetRawData;

/// Return value for gen_birth_points, containing the three birth positions.
#[derive(Debug, Clone)]
pub struct BirthPoints {
    /// Main spawn point on the planet surface
    pub birth_point: VectorF3,
    /// First resource point
    pub birth_resource_point0: VectorF3,
    /// Second resource point
    pub birth_resource_point1: VectorF3,
}

pub fn gen_birth_points(
    raw_data: &PlanetRawData,
    birth_seed: i32,
    radius: f32,
    star_direction: VectorF3,
) -> BirthPoints {
    // ---- main GenBirthPoints(PlanetRawData, int) body (C# lines 761-821) --
    let mut rand = DspRandom::new(birth_seed);

    // vector3_1 is the incoming star_direction (already normalised)
    let star_dir = star_direction.normalized();

    // C#: Vector3 normalized1 = Vector3.Cross(vector3_1, Vector3.up).normalized
    let mut basis1 = VectorF3::cross(&star_dir, &VectorF3::up()).normalized();
    // C#: Vector3 normalized2 = Vector3.Cross(normalized1, vector3_1).normalized
    let mut basis2 = VectorF3::cross(&basis1, &star_dir).normalized();

    let num2 = 256;

    // Outer loop – try up to 256 candidate birth directions
    for _ in 0..num2 {
        // C#: float num3 = (float)(dotNet35Random.NextDouble() * 2.0 - 1.0) * 0.5f
        let num3 = (rand.next_f64() * 2.0 - 1.0) as f32 * 0.5;
        // C#: float num4 = (float)(dotNet35Random.NextDouble() * 2.0 - 1.0) * 0.5f
        let num4 = (rand.next_f64() * 2.0 - 1.0) as f32 * 0.5;

        // C#: Vector3 vector3_2 = vector3_1 + num3 * normalized1 + num4 * normalized2
        let mut candidate = star_dir + basis1 * num3 + basis2 * num4;
        candidate.normalize();

        // C#: this.birthPoint = vector3_2 * (realRadius + 0.2f + 1.45f);
        //     0.2f + 1.45f = 1.65f, but keep the original constants for clarity
        let birth_point = candidate * (radius + 0.2 + 1.45);

        // C#: Vector3 vector3_3 = Vector3.Cross(vector3_2, Vector3.up);
        //     normalized1 = vector3_3.normalized;
        let cross_tmp = VectorF3::cross(&candidate, &VectorF3::up());
        basis1 = cross_tmp.normalized();
        // C#: vector3_3 = Vector3.Cross(normalized1, vector3_2);
        //     normalized2 = vector3_3.normalized;
        let cross_tmp2 = VectorF3::cross(&basis1, &candidate);
        basis2 = cross_tmp2.normalized();

        // Inner loop – try up to 10 resource-point offsets
        for _ in 0..10 {
            // C#: Vector2(x, y).normalized * 0.1f
            let v2x = (rand.next_f64() * 2.0 - 1.0) as f32;
            let v2y = (rand.next_f64() * 2.0 - 1.0) as f32;
            let v2len = (v2x * v2x + v2y * v2y).sqrt();
            let (v2x, v2y) = if v2len > 1e-10 {
                (v2x / v2len * 0.1, v2y / v2len * 0.1)
            } else {
                (0.0, 0.0)
            };

            // C#: Vector2 vector2_2 = -vector2_1;
            let v2x2 = -v2x;
            let v2y2 = -v2y;

            // C#: num5 = (float)(dotNet35Random.NextDouble() * 2.0 - 1.0) * 0.06f
            let num5 = (rand.next_f64() * 2.0 - 1.0) as f32 * 0.06;
            // C#: num6 = (float)(dotNet35Random.NextDouble() * 2.0 - 1.0) * 0.06f
            let num6 = (rand.next_f64() * 2.0 - 1.0) as f32 * 0.06;

            let v2x2 = v2x2 + num5;
            let v2y2 = v2y2 + num6;

            // C#: normalized3 = (vector3_2 + vector2_1.x * normalized1 + vector2_1.y * normalized2).normalized
            let rp0_dir = (candidate + basis1 * v2x + basis2 * v2y).normalized();

            // C#: normalized4 = (vector3_2 + vector2_2.x * normalized1 + vector2_2.y * normalized2).normalized
            let rp1_dir = (candidate + basis1 * v2x2 + basis2 * v2y2).normalized();

            // height threshold
            let height_threshold = radius + 0.2;

            // Use normalized variant since candidate, rp0_dir, rp1_dir are already unit-length
            if raw_data.query_height_normalized(&candidate) > height_threshold
                && raw_data.query_height_normalized(&rp0_dir) > height_threshold
                && raw_data.query_height_normalized(&rp1_dir) > height_threshold
            {
                // C#: check 8 surrounding offsets
                let vpos1 = rp0_dir + basis1 * 0.03;
                let vpos2 = rp0_dir - basis1 * 0.03;
                let vpos3 = rp0_dir + basis2 * 0.03;
                let vpos4 = rp0_dir - basis2 * 0.03;
                let vpos5 = rp1_dir + basis1 * 0.03;
                let vpos6 = rp1_dir - basis1 * 0.03;
                let vpos7 = rp1_dir + basis2 * 0.03;
                let vpos8 = rp1_dir - basis2 * 0.03;

                // Offset vectors are not unit-length; use the normalising variant
                if raw_data.query_height(&vpos1) > height_threshold
                    && raw_data.query_height(&vpos2) > height_threshold
                    && raw_data.query_height(&vpos3) > height_threshold
                    && raw_data.query_height(&vpos4) > height_threshold
                    && raw_data.query_height(&vpos5) > height_threshold
                    && raw_data.query_height(&vpos6) > height_threshold
                    && raw_data.query_height(&vpos7) > height_threshold
                    && raw_data.query_height(&vpos8) > height_threshold
                {
                    // Re‑normalise both resource-point directions
                    let rp0 = rp0_dir.normalized();
                    let rp1 = rp1_dir.normalized();

                    return BirthPoints {
                        birth_point,
                        birth_resource_point0: rp0,
                        birth_resource_point1: rp1,
                    };
                }
            }
        }
    }

    // ---- fallback (C# line 820) -------------------------------------------
    BirthPoints {
        birth_point: VectorF3(0.0, radius + 5.0, 0.0),
        birth_resource_point0: VectorF3::up(),
        birth_resource_point1: VectorF3::down(),
    }
}
