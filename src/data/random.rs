pub struct DspRandom {
    inext: usize,
    inextp: usize,
    seed_array: [i32; 56],
}

impl DspRandom {
    pub fn new(seed: i32) -> Self {
        let mut seed_array = [0; 56];
        // It would panic if seed==i32::MIN, what is just how game behaves.
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
        let update = |(lhs, rhs): (&mut i32, &i32)| {
            *lhs = lhs.wrapping_sub(*rhs);
            if lhs.is_negative() {
                *lhs += i32::MAX;
            }
        };
        for _ in 1..5 {
            // Write in form of iterators to hint compiler taking use of SIMD.
            unsafe {
                let ptr = (&mut seed_array as *mut [i32; 56]).cast::<i32>();
                // [1..24] subtract [32..55]
                let chunk_1 = &mut *ptr.add(1).cast::<[i32; 24]>();
                let chunk_2 = &*ptr.add(32).cast::<[i32; 24]>();
                chunk_1.iter_mut().zip(chunk_2.iter()).for_each(update);
                // [25..48] subtract [1..24]
                let chunk_3 = &mut *ptr.add(25).cast::<[i32; 24]>();
                chunk_3.iter_mut().zip(chunk_1.iter()).for_each(update);
                // [49..55] subtract [25..31]
                let chunk_4 = &*ptr.add(25).cast::<[i32; 7]>();
                let chunk_5 = &mut *ptr.add(49).cast::<[i32; 7]>();
                chunk_5.iter_mut().zip(chunk_4.iter()).for_each(update);
            }
        }

        Self {
            inext: 0,
            inextp: 31,
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
