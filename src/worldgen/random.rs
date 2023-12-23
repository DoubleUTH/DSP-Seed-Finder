const MBIG: i32 = 2147483647;

pub struct DspRandom {
    inext: usize,
    inextp: usize,
    pub seed: i32,
    seed_array: [i32; 56],
}

impl DspRandom {
    pub fn new(seed: i32) -> Self {
        let mut seed_array = [0; 56];
        let mut num1 = 161803398 - seed.abs();
        seed_array[55] = num1;
        let mut num2 = 1;
        for index1 in 1..55 {
            let index2 = (21 * index1) % 55;
            seed_array[index2] = num2;
            num2 = num1 - num2;
            if num2 < 0 {
                num2 += MBIG;
            }
            num1 = seed_array[index2]
        }
        for _index3 in 1..5 {
            for index4 in 1..56 {
                let val = (seed_array[index4] as i64) - (seed_array[1 + (index4 + 30) % 55] as i64);
                seed_array[index4] = val as i32;
                if seed_array[index4] < 0 {
                    seed_array[index4] += MBIG;
                }
            }
        }

        Self {
            inext: 0,
            inextp: 31,
            seed,
            seed_array,
        }
    }

    fn sample(&mut self) -> f64 {
        self.inext += 1;
        if self.inext >= 56 {
            self.inext = 1
        }
        self.inextp += 1;
        if self.inextp >= 56 {
            self.inextp = 1
        }
        let mut num = self.seed_array[self.inext] - self.seed_array[self.inextp];
        if num < 0 {
            num += MBIG;
        }
        self.seed_array[self.inext] = num;
        (num as f64) * (1.0 / (MBIG as f64))
    }

    pub fn next_f64(&mut self) -> f64 {
        self.sample()
    }

    pub fn next_f32(&mut self) -> f32 {
        self.sample() as f32
    }

    pub fn next_i32(&mut self, min_value: i32, max_value: i32) -> i32 {
        let num = (max_value - min_value) as u32;
        if num <= 1 {
            min_value
        } else {
            (((self.sample() * (num as f64)) as i64) + (min_value as i64)) as i32
        }
    }

    pub fn next_usize(&mut self) -> usize {
        (self.sample() * (MBIG as f64)) as usize
    }

    pub fn next_seed(&mut self) -> i32 {
        (self.sample() * (MBIG as f64)) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::worldgen::random::DspRandom;

    #[test]
    fn rand_test_1() {
        let mut rand = DspRandom::new(1);
        assert_eq!(rand.next_f64(), 0.36685459100029205);
        assert_eq!(rand.next_f64(), 0.20793473031741336);
        assert_eq!(rand.next_f64(), 0.9534165486476468);
        assert_eq!(rand.next_f64(), 0.2524418142868402);
        assert_eq!(rand.next_f64(), 0.9074322701932087);
    }

    #[test]
    fn rand_test_2() {
        let mut rand = DspRandom::new(1575693681);
        assert_eq!(rand.next_f64(), 0.7679300078972854);
        assert_eq!(rand.next_f64(), 0.7785721038368406);
        assert_eq!(rand.next_f64(), 0.7108933994131602);
        assert_eq!(rand.next_f64(), 0.2166100252497058);
        assert_eq!(rand.next_f64(), 0.27458891844124966);
    }
}
