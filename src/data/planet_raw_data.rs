use super::vector_f3::VectorF3;
use std::f64::consts::PI;
use std::sync::OnceLock;

// C# private static Vector3[] verts200 / verts80
static VERTS_200: OnceLock<Vec<VectorF3>> = OnceLock::new();
static VERTS_80: OnceLock<Vec<VectorF3>> = OnceLock::new();
static INDEX_MAP_200: OnceLock<Vec<i32>> = OnceLock::new();
static INDEX_MAP_80: OnceLock<Vec<i32>> = OnceLock::new();

/// C# public static Vector3[] poles
static POLES: [VectorF3; 6] = [
    VectorF3(1.0, 0.0, 0.0),  // right
    VectorF3(-1.0, 0.0, 0.0), // left
    VectorF3(0.0, 1.0, 0.0),  // up
    VectorF3(0.0, -1.0, 0.0), // down
    VectorF3(0.0, 0.0, 1.0),  // forward
    VectorF3(0.0, 0.0, -1.0), // back
];

#[derive(Debug, Clone)]
pub struct PlanetRawData {
    pub precision: usize,
    pub vertices: Vec<VectorF3>,
    pub index_map: Vec<i32>,
    index_map_precision: usize,
    index_map_face_stride: usize,
    index_map_corner_stride: usize,
    index_map_data_length: usize,
}

impl PlanetRawData {
    // ------------------------------------------------------------------
    //  constructor  (C# lines 46–63)
    // ------------------------------------------------------------------
    pub fn new(precision: usize) -> Self {
        let data_length = (precision + 1) * (precision + 1) * 4;
        let imp = precision >> 2; // indexMapPrecision = precision >> 2
        let imfs = imp * imp; // indexMapFaceStride
        let imcs = imfs * 3; // indexMapCornerStride
        let imdl = imcs * 8; // indexMapDataLength

        Self {
            precision,
            vertices: vec![VectorF3::zero(); data_length],
            index_map: vec![-1i32; imdl],
            index_map_precision: imp,
            index_map_face_stride: imfs,
            index_map_corner_stride: imcs,
            index_map_data_length: imdl,
        }
    }

    // ------------------------------------------------------------------
    //  properties  (C# lines 94–98)
    // ------------------------------------------------------------------
    /// (precision + 1)² * 4
    pub fn data_length(&self) -> usize {
        (self.precision + 1) * (self.precision + 1) * 4
    }

    /// (precision + 1) * 2
    pub fn stride(&self) -> usize {
        (self.precision + 1) * 2
    }

    /// precision + 1
    pub fn substride(&self) -> usize {
        self.precision + 1
    }

    // ------------------------------------------------------------------
    //  trans  (C# lines 433–439)
    //  C# signature: private int trans(float x, int pr)
    //  C# body uses (double) casts; we keep f64 arithmetic then cast.
    // ------------------------------------------------------------------
    fn trans(x: f32, pr: usize) -> usize {
        let num = ((x as f64 + 0.23f64).sqrt() - 0.47958320379257204f64) / 0.62947052717208862f64
            * (pr as f64);
        let idx = num as usize;
        if idx >= pr {
            pr - 1
        } else {
            idx
        }
    }

    // ------------------------------------------------------------------
    //  PositionHash  (C# lines 441–475)
    //  C# signature: public int PositionHash(Vector3 v, int corner = 0)
    //  Note: C# Vector3 uses float, ours uses f64. We preserve the
    //  (double) casts from the C# by staying in f64.
    // ------------------------------------------------------------------
    pub fn position_hash(&self, v: &VectorF3, corner: usize) -> usize {
        let corner = if corner == 0 {
            ((v.0 > 0.0) as usize) + ((v.1 > 0.0) as usize) * 2 + ((v.2 > 0.0) as usize) * 4
        } else {
            corner
        };

        let vx = if v.0 < 0.0 { -v.0 } else { v.0 };
        let vy = if v.1 < 0.0 { -v.1 } else { v.1 };
        let vz = if v.2 < 0.0 { -v.2 } else { v.2 };

        // C#: if ((double)v.x < 1E-06 && (double)v.y < 1E-06 && (double)v.z < 1E-06) return 0;
        if vx < 1e-6 && vy < 1e-6 && vz < 1e-6 {
            return 0;
        }

        let n1: usize;
        let n2: usize;
        let n3: usize;

        if vx >= vy && vx >= vz {
            n1 = 0;
            n2 = Self::trans((vz / vx) as f32, self.index_map_precision);
            n3 = Self::trans((vy / vx) as f32, self.index_map_precision);
        } else if vy >= vx && vy >= vz {
            n1 = 1;
            n2 = Self::trans((vx / vy) as f32, self.index_map_precision);
            n3 = Self::trans((vz / vy) as f32, self.index_map_precision);
        } else {
            n1 = 2;
            n2 = Self::trans((vx / vz) as f32, self.index_map_precision);
            n3 = Self::trans((vy / vz) as f32, self.index_map_precision);
        }

        n2 + n3 * self.index_map_precision
            + n1 * self.index_map_face_stride
            + corner * self.index_map_corner_stride
    }

    // ------------------------------------------------------------------
    //  CalcVerts  (C# lines 100–224)
    // ------------------------------------------------------------------
    /// Generates `vertices` (unit-sphere positions) and `index_map`
    /// (spatial hash lookup) from `precision`.
    pub fn calc_verts(&mut self) {
        // cache hit – precision 200
        if self.precision == 200 {
            if let (Some(verts), Some(map)) = (VERTS_200.get(), INDEX_MAP_200.get()) {
                self.vertices.copy_from_slice(verts);
                self.index_map.copy_from_slice(map);
                return;
            }
        }
        // cache hit – precision 80
        if self.precision == 80 {
            if let (Some(verts), Some(map)) = (VERTS_80.get(), INDEX_MAP_80.get()) {
                self.vertices.copy_from_slice(verts);
                self.index_map.copy_from_slice(map);
                return;
            }
        }

        // --- compute ------------------------------------------------------------------
        let data_len = self.data_length();
        let stride_val = self.stride();
        let sub = self.substride();

        // initialise index_map to -1
        for v in self.index_map.iter_mut() {
            *v = -1;
        }

        for i in 0..data_len {
            // C#: int num3 = index1 % num1;   (num1 = stride)
            let n3 = i % stride_val;
            // C#: int num4 = index1 / num1;
            let n4 = i / stride_val;
            // C#: int num5 = num3 % num2;      (num2 = substride)
            let n5 = n3 % sub;
            // C#: int num6 = num4 % num2;
            let n6 = n4 % sub;

            // C#: int num7 = ((num3 >= num2 ? 1 : 0) + (num4 >= num2 ? 1 : 0) * 2) * 2
            //               + (num5 >= num6 ? 0 : 1);
            let n7 = (((n3 >= sub) as usize) + ((n4 >= sub) as usize) * 2) * 2
                + if n5 >= n6 { 0 } else { 1 };

            // C#: float num8 = num5 >= num6 ? (float)(precision - num5) : (float)num5;
            // (C# casts to float; we use f32)
            let n8: f32 = if n5 >= n6 {
                (self.precision - n5) as f32
            } else {
                n5 as f32
            };
            // C#: float num9 = num5 >= num6 ? (float)num6 : (float)(precision - num6);
            let n9: f32 = if n5 >= n6 {
                n6 as f32
            } else {
                (self.precision - n6) as f32
            };

            // C#: float num10 = (float)precision - num9;
            let n10: f32 = self.precision as f32 - n9;
            // C#: float t1 = num9 / (float)precision;
            let t1: f32 = n9 / self.precision as f32;
            // C#: float t2 = (double)num10 > 0.0 ? num8 / num10 : 0.0f;
            let t2: f32 = if n10 > 0.0f32 { n8 / n10 } else { 0.0f32 };

            let (pole1, pole2, pole3, corner): (&VectorF3, &VectorF3, &VectorF3, usize) = match n7 {
                0 => (&POLES[2], &POLES[0], &POLES[4], 7),
                1 => (&POLES[3], &POLES[4], &POLES[0], 5),
                2 => (&POLES[2], &POLES[4], &POLES[1], 6),
                3 => (&POLES[3], &POLES[1], &POLES[4], 4),
                4 => (&POLES[2], &POLES[1], &POLES[5], 2),
                5 => (&POLES[3], &POLES[5], &POLES[1], 0),
                6 => (&POLES[2], &POLES[5], &POLES[0], 3),
                7 => (&POLES[3], &POLES[0], &POLES[5], 1),
                _ => (&POLES[2], &POLES[0], &POLES[4], 7),
            };

            // C#: this.vertices[index1] = Vector3.Slerp(
            //        Vector3.Slerp(pole1, pole3, t1),
            //        Vector3.Slerp(pole2, pole3, t1), t2);
            let slerp_a = VectorF3::slerp(pole1, pole3, t1);
            let slerp_b = VectorF3::slerp(pole2, pole3, t1);
            self.vertices[i] = VectorF3::slerp(&slerp_a, &slerp_b, t2);

            // C#: int index2 = this.PositionHash(this.vertices[index1], corner);
            let idx2 = self.position_hash(&self.vertices[i], corner);
            if self.index_map[idx2] == -1 {
                self.index_map[idx2] = i as i32;
            }
        }

        // C#: forward-fill empty slots
        for i in 1..self.index_map_data_length {
            if self.index_map[i] == -1 {
                self.index_map[i] = self.index_map[i - 1];
            }
        }

        // --- store in static caches ---
        if self.precision == 200 {
            let _ = VERTS_200.set(self.vertices.clone());
            let _ = INDEX_MAP_200.set(self.index_map.clone());
        } else if self.precision == 80 {
            let _ = VERTS_80.set(self.vertices.clone());
            let _ = INDEX_MAP_80.set(self.index_map.clone());
        }
    }

    // ------------------------------------------------------------------
    //  QueryHeight  (C# lines 317–348)
    // ------------------------------------------------------------------
    /// Inverse-distance-weighted height interpolation.
    ///
    /// `algo` provides lazy height values via `get_height()`.
    /// Returns interpolated height in game units.
    pub fn query_height(
        &self,
        vpos: &VectorF3,
        algo: &dyn super::planet_algorithms::PlanetAlgorithm,
    ) -> f32 {
        let mut vpos = vpos.clone();
        vpos.normalize();

        let data_len = self.data_length();
        let stride_val = self.stride();

        let index1 = self.index_map[self.position_hash(&vpos, 0)];

        // C#: float num1 = (float)(3.1415927410125732 / (double)(precision * 2)
        //                           * 1.2000000476837158);
        let num1: f64 = (PI / (self.precision as f64 * 2.0)) * 1.2_f64;
        let num2: f64 = num1 * num1;

        let mut num3: f32 = 0.0f32; // weight sum
        let mut num4: f32 = 0.0f32; // weighted height sum

        for i2 in -1..=3 {
            for i3 in -1_i32..=3 {
                let idx4 = index1
                    .wrapping_add(i2)
                    .wrapping_add(i3.wrapping_mul(stride_val as i32))
                    as usize;
                if idx4 < data_len {
                    // C#: float sqrMagnitude = (vertices[index4] - vpos).sqrMagnitude;
                    let sqr_mag = self.vertices[idx4].distance_sq_from(&vpos);
                    // C#: if ((double)sqrMagnitude <= (double)num2)
                    if (sqr_mag as f64) <= num2 {
                        // C#: float num5 = (float)(1.0 - (double)Mathf.Sqrt(sqrMagnitude)
                        //                           / (double)num1);
                        let num5 = 1.0f32 - (sqr_mag.sqrt() / num1 as f32);
                        let num6 = algo.get_height(idx4, self);
                        num3 += num5;
                        num4 += num6 * num5;
                    }
                }
            }
        }

        if num3 != 0.0f32 {
            num4 / num3
        } else {
            algo.get_height(0, self)
        }
    }
}
