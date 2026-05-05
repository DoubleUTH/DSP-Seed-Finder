#[derive(Debug)]
pub struct DspRandom {
    inext: usize,
    inextp: usize,
    seed_array: [i32; 56],
}

impl DspRandom {
    pub fn new(seed: i32) -> Self {
        let mut seed_array = [0; 56];
        let mut num1 = 161803398 - seed.abs();
        seed_array[55] = num1;
        let mut num2 = 1;
        let mut index2 = 0;
        for _ in 1..55 {
            index2 += 21;
            if index2 >= 55 {
                index2 -= 55;
            }
            seed_array[index2] = num2;
            num2 = num1 - num2;
            if num2 < 0 {
                num2 += i32::MAX;
            }
            num1 = seed_array[index2]
        }

        let ptr = seed_array.as_mut_ptr();

        let (chunk1_lhs, chunk1_rhs, chunk2_lhs, chunk2_rhs) = unsafe {
            (
                &mut *ptr.add(1).cast::<[i32; 24]>(),
                &*ptr.add(32).cast::<[i32; 24]>(),
                &mut *ptr.add(25).cast::<[i32; 31]>(),
                &*ptr.add(1).cast::<[i32; 31]>(),
            )
        };

        let update = |(lhs, rhs): (&mut i32, &i32)| {
            *lhs = lhs.wrapping_sub(*rhs);
            if lhs.is_negative() {
                *lhs += i32::MAX;
            }
        };

        for _ in 1..5 {
            chunk1_lhs.iter_mut().zip(chunk1_rhs).for_each(update);
            chunk2_lhs.iter_mut().zip(chunk2_rhs).for_each(update);
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
