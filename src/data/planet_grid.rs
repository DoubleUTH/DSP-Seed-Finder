use super::vector_f3::VectorF3;
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
//  Constants – hardcoded for precision = 200
// ---------------------------------------------------------------------------
pub const PRECISION: usize = 200;
pub const DATA_LENGTH: usize = (PRECISION + 1) * (PRECISION + 1) * 4; // 161604
pub const STRIDE: i32 = ((PRECISION + 1) * 2) as i32; // 402
const SUBSTRIDE: usize = PRECISION + 1; // 201
const INDEX_MAP_PRECISION: usize = PRECISION >> 2; // 50
const INDEX_MAP_FACE_STRIDE: usize = INDEX_MAP_PRECISION * INDEX_MAP_PRECISION; // 2500
const INDEX_MAP_CORNER_STRIDE: usize = INDEX_MAP_FACE_STRIDE * 3; // 7500
const INDEX_MAP_DATA_LENGTH: usize = INDEX_MAP_CORNER_STRIDE * 8; // 60000

// ---------------------------------------------------------------------------
//  Pole constants (C# public static Vector3[] poles)
// ---------------------------------------------------------------------------
static POLES: [VectorF3; 6] = [
    VectorF3::right(),
    VectorF3::left(),
    VectorF3::up(),
    VectorF3::down(),
    VectorF3::forward(),
    VectorF3::back(),
];

pub struct PlanetGrid {
    pub vertices: Vec<VectorF3>,
    pub index_map: Vec<i32>,
}

impl PlanetGrid {
    pub fn get_vertex(&self, index: usize) -> &VectorF3 {
        unsafe { &self.vertices.get_unchecked(index) }
    }
}

static PLANET_GRID: OnceLock<PlanetGrid> = OnceLock::new();

pub fn get_planet_grid() -> &'static PlanetGrid {
    PLANET_GRID.get_or_init(|| {
        let mut vertices = vec![VectorF3::zero(); DATA_LENGTH];
        let mut index_map = vec![-1i32; INDEX_MAP_DATA_LENGTH];

        for i in 0..DATA_LENGTH {
            // C#: int num3 = index1 % num1;   (num1 = stride)
            let n3 = i % STRIDE as usize;
            // C#: int num4 = index1 / num1;
            let n4 = i / STRIDE as usize;
            // C#: int num5 = num3 % num2;      (num2 = substride)
            let n5 = n3 % SUBSTRIDE;
            // C#: int num6 = num4 % num2;
            let n6 = n4 % SUBSTRIDE;

            // C#: int num7 = ((num3 >= num2 ? 1 : 0) + (num4 >= num2 ? 1 : 0) * 2) * 2
            //               + (num5 >= num6 ? 0 : 1);
            let n7 = (((n3 >= SUBSTRIDE) as usize) + ((n4 >= SUBSTRIDE) as usize) * 2) * 2
                + if n5 >= n6 { 0 } else { 1 };

            let n8: f32 = if n5 >= n6 {
                (PRECISION - n5) as f32
            } else {
                n5 as f32
            };
            let n9: f32 = if n5 >= n6 {
                n6 as f32
            } else {
                (PRECISION - n6) as f32
            };

            let n10: f32 = PRECISION as f32 - n9;
            let t1: f32 = n9 / PRECISION as f32;
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

            let slerp_a = VectorF3::slerp(pole1, pole3, t1);
            let slerp_b = VectorF3::slerp(pole2, pole3, t1);
            let vert = VectorF3::slerp(&slerp_a, &slerp_b, t2);

            // C#: int index2 = this.PositionHash(this.vertices[index1], corner);
            let idx2 = position_hash(&vert, corner);
            if index_map[idx2] == -1 {
                index_map[idx2] = i as i32;
            }

            vertices[i] = vert;
        }

        // C#: forward-fill empty slots
        for i in 1..INDEX_MAP_DATA_LENGTH {
            if index_map[i] == -1 {
                index_map[i] = index_map[i - 1];
            }
        }

        PlanetGrid {
            vertices,
            index_map,
        }
    })
}

// ---------------------------------------------------------------------------
//  trans  (C# lines 433–439)
// ---------------------------------------------------------------------------
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

// ---------------------------------------------------------------------------
//  position_hash (internal)  (C# lines 441–475)
// ---------------------------------------------------------------------------
pub fn position_hash(v: &VectorF3, corner: usize) -> usize {
    let corner = if corner == 0 {
        ((v.0 > 0.0) as usize) + ((v.1 > 0.0) as usize) * 2 + ((v.2 > 0.0) as usize) * 4
    } else {
        corner
    };

    let vx = if v.0 < 0.0 { -v.0 } else { v.0 };
    let vy = if v.1 < 0.0 { -v.1 } else { v.1 };
    let vz = if v.2 < 0.0 { -v.2 } else { v.2 };

    if vx < 1e-6 && vy < 1e-6 && vz < 1e-6 {
        return 0;
    }

    let n1: usize;
    let n2: usize;
    let n3: usize;

    if vx >= vy && vx >= vz {
        n1 = 0;
        n2 = trans((vz / vx) as f32, INDEX_MAP_PRECISION);
        n3 = trans((vy / vx) as f32, INDEX_MAP_PRECISION);
    } else if vy >= vx && vy >= vz {
        n1 = 1;
        n2 = trans((vx / vy) as f32, INDEX_MAP_PRECISION);
        n3 = trans((vz / vy) as f32, INDEX_MAP_PRECISION);
    } else {
        n1 = 2;
        n2 = trans((vx / vz) as f32, INDEX_MAP_PRECISION);
        n3 = trans((vy / vz) as f32, INDEX_MAP_PRECISION);
    }

    n2 + n3 * INDEX_MAP_PRECISION + n1 * INDEX_MAP_FACE_STRIDE + corner * INDEX_MAP_CORNER_STRIDE
}
