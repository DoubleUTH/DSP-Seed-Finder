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
                num2 += i32::MAX;
            }
            num1 = seed_array[index2]
        }
        for _index3 in 1..5 {
            for index4 in 1..56 {
                let mut val = seed_array[index4].wrapping_sub(seed_array[1 + (index4 + 30) % 55]);
                if val < 0 {
                    val += i32::MAX;
                }
                seed_array[index4] = val;
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
            num += i32::MAX;
        }
        self.seed_array[self.inext] = num;
        (num as f64) * (1.0 / (i32::MAX as f64))
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        self.sample()
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        self.sample() as f32
    }

    #[inline]
    pub fn next_i32(&mut self, max_value: i32) -> i32 {
        (self.sample() * (max_value as f64)) as i32
    }

    #[inline]
    pub fn next_usize(&mut self) -> usize {
        (self.sample() * (i32::MAX as f64)) as usize
    }

    #[inline]
    pub fn next_seed(&mut self) -> i32 {
        (self.sample() * (i32::MAX as f64)) as i32
    }
}
