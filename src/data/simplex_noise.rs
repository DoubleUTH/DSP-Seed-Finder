/// Port of SimplexNoise from DSP game (SimpleNoise.cs)
/// Uses DspRandom for seed-based permutation shuffling.
use super::random::DspRandom;

// ---- Static gradient tables (constant) ----

const GRAD3: [[f64; 3]; 12] = [
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
    [1.0, -1.0, 0.0],
    [-1.0, -1.0, 0.0],
    [1.0, 0.0, 1.0],
    [-1.0, 0.0, 1.0],
    [1.0, 0.0, -1.0],
    [-1.0, 0.0, -1.0],
    [0.0, 1.0, 1.0],
    [0.0, -1.0, 1.0],
    [0.0, 1.0, -1.0],
    [0.0, -1.0, -1.0],
];

// ---- Constants ----
const F3: f64 = 1.0 / 3.0;
const G3: f64 = 1.0 / 6.0;

/// SimplexNoise implementation with DSP-specific permutation.
pub struct SimplexNoise {
    perm: [i16; 512],
    perm_mod12: [i16; 512],
}

impl SimplexNoise {
    /// Create with seed-shuffled permutation.
    pub fn with_seed(seed: i32) -> Self {
        let mut noise = Self {
            perm: [0; 512],
            perm_mod12: [0; 512],
        };
        noise.init_member_seed(seed);
        noise
    }

    /// Initialize with seed-shuffled permutation (using DspRandom).
    fn init_member_seed(&mut self, seed: i32) {
        let mut p: [i16; 256] = std::array::from_fn(|i| i as i16);

        // Only shuffle when seed != 0 (matching C# behavior where seed 0 might be special)
        // Actually the C# always shuffles. Let's shuffle based on seed.
        let mut rand = DspRandom::new(seed);
        for i1 in 0..256 {
            let i2 = rand.next_i32(256) as usize;
            let num = p[i1];
            p[i1] = p[i2];
            p[i2] = num;
        }

        for i in 0..512 {
            self.perm[i] = p[i & 255];
            self.perm_mod12[i] = (self.perm[i] % 12) as i16;
        }
    }

    /// 3D simplex noise.
    pub fn noise_3d(&self, xin: f64, yin: f64, zin: f64) -> f64 {
        let num1 = (xin + yin + zin) * F3;
        let num2 = fastfloor(xin + num1);
        let num3 = fastfloor(yin + num1);
        let num4 = fastfloor(zin + num1);
        let num5 = (num2 + num3 + num4) as f64 * G3;
        let num6 = num2 as f64 - num5;
        let num7 = num3 as f64 - num5;
        let num8 = num4 as f64 - num5;
        let x1 = xin - num6;
        let y1 = yin - num7;
        let z1 = zin - num8;

        let (n9, n10, n11, n12, n13, n14) = if x1 >= y1 {
            if y1 >= z1 {
                (1, 0, 0, 1, 1, 0)
            } else if x1 >= z1 {
                (1, 0, 0, 1, 0, 1)
            } else {
                (0, 0, 1, 1, 0, 1)
            }
        } else if y1 < z1 {
            (0, 0, 1, 0, 1, 1)
        } else if x1 < z1 {
            (0, 1, 0, 0, 1, 1)
        } else {
            (0, 1, 0, 1, 1, 0)
        };

        let x2 = x1 - n9 as f64 + G3;
        let y2 = y1 - n10 as f64 + G3;
        let z2 = z1 - n11 as f64 + G3;
        let x3 = x1 - n12 as f64 + 2.0 * G3;
        let y3 = y1 - n13 as f64 + 2.0 * G3;
        let z3 = z1 - n14 as f64 + 2.0 * G3;
        let x4 = x1 - 1.0 + 3.0 * G3;
        let y4 = y1 - 1.0 + 3.0 * G3;
        let z4 = z1 - 1.0 + 3.0 * G3;

        let n15 = (num2 & 255) as usize;
        let n16 = (num3 & 255) as usize;
        let idx1 = (num4 & 255) as usize;

        let index2 = self.perm_mod12
            [(n15 + self.perm[(n16 + self.perm[idx1] as usize) as usize] as usize) as usize]
            as usize;
        let index3 = self.perm_mod12[(n15
            + n9
            + self.perm[(n16 + n10 + self.perm[(idx1 + n11) as usize] as usize) as usize] as usize)
            as usize] as usize;
        let index4 = self.perm_mod12[(n15
            + n12
            + self.perm[(n16 + n13 + self.perm[(idx1 + n14) as usize] as usize) as usize] as usize)
            as usize] as usize;
        let index5 = self.perm_mod12[(n15
            + 1
            + self.perm[(n16 + 1 + self.perm[(idx1 + 1) as usize] as usize) as usize] as usize)
            as usize] as usize;

        let n17 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
        let n18 = if n17 < 0.0 {
            0.0
        } else {
            let n19 = n17 * n17;
            n19 * n19 * dot3_3d(&GRAD3[index2], x1, y1, z1)
        };

        let n20 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
        let n21 = if n20 < 0.0 {
            0.0
        } else {
            let n22 = n20 * n20;
            n22 * n22 * dot3_3d(&GRAD3[index3], x2, y2, z2)
        };

        let n23 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
        let n24 = if n23 < 0.0 {
            0.0
        } else {
            let n25 = n23 * n23;
            n25 * n25 * dot3_3d(&GRAD3[index4], x3, y3, z3)
        };

        let n26 = 0.6 - x4 * x4 - y4 * y4 - z4 * z4;
        let n27 = if n26 < 0.0 {
            0.0
        } else {
            let n28 = n26 * n26;
            n28 * n28 * dot3_3d(&GRAD3[index5], x4, y4, z4)
        };

        32.696434 * (n18 + n21 + n24 + n27)
    }

    // ---- FBM (Fractal Brownian Motion) variants ----

    /// 3D FBM noise.
    pub fn noise_3d_fbm(
        &self,
        x: f64,
        y: f64,
        z: f64,
        n_octaves: i32,
        delta_amp: f64,
        delta_wlen: f64,
    ) -> f64 {
        let mut num1 = 0.0;
        let mut num2 = 0.5;
        let mut x = x;
        let mut y = y;
        let mut z = z;
        for _ in 0..n_octaves {
            num1 += self.noise_3d(x, y, z) * num2;
            num2 *= delta_amp;
            x *= delta_wlen;
            y *= delta_wlen;
            z *= delta_wlen;
        }
        num1
    }

    /// 3D FBM noise with custom initial amplitude.
    pub fn noise_3d_fbm_initial_amp(
        &self,
        x: f64,
        y: f64,
        z: f64,
        n_octaves: i32,
        delta_amp: f64,
        delta_wlen: f64,
        initial_amp: f64,
    ) -> f64 {
        let mut num1 = 0.0;
        let mut num2 = initial_amp;
        let mut x = x;
        let mut y = y;
        let mut z = z;
        for _ in 0..n_octaves {
            num1 += self.noise_3d(x, y, z) * num2;
            num2 *= delta_amp;
            x *= delta_wlen;
            y *= delta_wlen;
            z *= delta_wlen;
        }
        num1
    }

    /// 3D ridged noise.
    pub fn ridged_noise(
        &self,
        x: f64,
        y: f64,
        z: f64,
        n_octaves: i32,
        delta_amp: f64,
        delta_wlen: f64,
        initial_amp: f64,
    ) -> f64 {
        let mut num1 = 0.0;
        let mut num2 = initial_amp;
        let mut x = x;
        let mut y = y;
        let mut z = z;
        for _ in 0..n_octaves {
            num1 += (self.noise_3d(x, y, z) * num2).abs();
            num2 *= delta_amp;
            x *= delta_wlen;
            y *= delta_wlen;
            z *= delta_wlen;
        }
        num1
    }
}

// ---- Helper functions ----

#[inline]
fn fastfloor(x: f64) -> i32 {
    let num = x as i32;
    if x >= num as f64 {
        num
    } else {
        num - 1
    }
}

#[inline]
fn dot3_3d(g: &[f64; 3], x: f64, y: f64, z: f64) -> f64 {
    g[0] * x + g[1] * y + g[2] * z
}
