use super::vector_f3::VectorF3;
use std::cell::RefCell;
use std::f64::consts::PI;

// ---------------------------------------------------------------------------
//  Constants – hardcoded for precision = 200
// ---------------------------------------------------------------------------
const PRECISION: usize = 200;
const DATA_LENGTH: usize = (PRECISION + 1) * (PRECISION + 1) * 4; // 161604
const STRIDE: usize = (PRECISION + 1) * 2; // 402
const SUBSTRIDE: usize = PRECISION + 1; // 201
const INDEX_MAP_PRECISION: usize = PRECISION >> 2; // 50
const INDEX_MAP_FACE_STRIDE: usize = INDEX_MAP_PRECISION * INDEX_MAP_PRECISION; // 2500
const INDEX_MAP_CORNER_STRIDE: usize = INDEX_MAP_FACE_STRIDE * 3; // 7500
const INDEX_MAP_DATA_LENGTH: usize = INDEX_MAP_CORNER_STRIDE * 8; // 60000

// ---------------------------------------------------------------------------
//  Pole constants (C# public static Vector3[] poles)
// ---------------------------------------------------------------------------
static POLES: [VectorF3; 6] = [
    VectorF3(1.0, 0.0, 0.0),  // right
    VectorF3(-1.0, 0.0, 0.0), // left
    VectorF3(0.0, 1.0, 0.0),  // up
    VectorF3(0.0, -1.0, 0.0), // down
    VectorF3(0.0, 0.0, 1.0),  // forward
    VectorF3(0.0, 0.0, -1.0), // back
];

// ---------------------------------------------------------------------------
//  Thread-local cache – computed once per thread
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct PlanetRawData {
    pub vertices: Vec<VectorF3>,
    pub index_map: Vec<i32>,
}

thread_local! {
    static PLANET_RAW_DATA_200: RefCell<Option<PlanetRawData>> = RefCell::new(None);
}

// ---------------------------------------------------------------------------
//  get_vertex – thread-safe access to a single vertex
// ---------------------------------------------------------------------------
pub fn get_vertex(index: usize) -> &'static VectorF3 {
    // Safety: the PlanetRawData is stored in a thread_local and never moves
    // once initialized. We return a reference with a bounded lifetime.
    PLANET_RAW_DATA_200.with(|cell| {
        let opt = cell.borrow();
        // We must extend the lifetime from the RefGuard to 'static.
        // This is sound because the data is never deallocated within the thread.
        let ptr: *const VectorF3 = &opt.as_ref().unwrap().vertices[index];
        unsafe { &*ptr }
    })
}

// ---------------------------------------------------------------------------
//  calc_verts – compute vertices & index_map, store in thread-local
// ---------------------------------------------------------------------------
fn init_raw_data() {
    PLANET_RAW_DATA_200.with(|cell| {
        if cell.borrow().is_some() {
            return;
        }

        let mut vertices = vec![VectorF3::zero(); DATA_LENGTH];
        let mut index_map = vec![-1i32; INDEX_MAP_DATA_LENGTH];

        for i in 0..DATA_LENGTH {
            // C#: int num3 = index1 % num1;   (num1 = stride)
            let n3 = i % STRIDE;
            // C#: int num4 = index1 / num1;
            let n4 = i / STRIDE;
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
            let idx2 = position_hash_impl(&vert, corner);
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

        cell.replace(Some(PlanetRawData {
            vertices,
            index_map,
        }));
    });
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
fn position_hash_impl(v: &VectorF3, corner: usize) -> usize {
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

/// Returns a reference to the thread-local `PlanetRawData`, initializing it on first access.
pub fn get_raw_data() -> &'static PlanetRawData {
    init_raw_data();
    PLANET_RAW_DATA_200.with(|cell| {
        let opt = cell.borrow();
        // Safety: the data is never deallocated within the thread after initialization.
        let ptr: *const PlanetRawData = opt.as_ref().unwrap();
        unsafe { &*ptr }
    })
}

// ---------------------------------------------------------------------------
//  Core height interpolation (shared by both query variants)
// ---------------------------------------------------------------------------
fn query_height_impl(
    vpos_normalized: &VectorF3,
    algo: &dyn super::planet_algorithms::PlanetAlgorithm,
    raw_data: &PlanetRawData,
) -> f32 {
    let index1 = raw_data.index_map[position_hash_impl(vpos_normalized, 0)];

    let num1: f64 = (PI / (PRECISION as f64 * 2.0)) * 1.2_f64;
    let num2: f64 = num1 * num1;

    let mut num3: f32 = 0.0f32;
    let mut num4: f32 = 0.0f32;

    for i2 in -1..=3 {
        for i3 in -1_i32..=3 {
            let idx4 = index1
                .wrapping_add(i2)
                .wrapping_add(i3.wrapping_mul(STRIDE as i32)) as usize;
            if idx4 < DATA_LENGTH {
                let sqr_mag = raw_data.vertices[idx4].distance_sq_from(vpos_normalized);
                if (sqr_mag as f64) <= num2 {
                    let num5 = 1.0f32 - (sqr_mag.sqrt() / num1 as f32);
                    let num6 = algo.get_height(idx4);
                    num3 += num5;
                    num4 += num6 * num5;
                }
            }
        }
    }

    if num3 != 0.0f32 {
        num4 / num3
    } else {
        algo.get_height(0)
    }
}

// ---------------------------------------------------------------------------
//  query_height  (C# lines 317–348)
// ---------------------------------------------------------------------------
/// Queries the interpolated height at `vpos`, normalizing the input first.
/// Prefer `query_height_normalized` when the input is already unit-length.
pub fn query_height(
    vpos: &VectorF3,
    algo: &dyn super::planet_algorithms::PlanetAlgorithm,
    raw_data: &PlanetRawData,
) -> f32 {
    let mut vpos = vpos.clone();
    vpos.normalize();
    query_height_impl(&vpos, algo, raw_data)
}

/// Same as `query_height`, but assumes `vpos_normalized` is already a unit vector.
/// This avoids the redundant normalisation and clone.
pub fn query_height_normalized(
    vpos_normalized: &VectorF3,
    algo: &dyn super::planet_algorithms::PlanetAlgorithm,
    raw_data: &PlanetRawData,
) -> f32 {
    query_height_impl(vpos_normalized, algo, raw_data)
}
