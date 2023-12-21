#[derive(Debug, Clone, Copy)]
pub struct GameDesc {
    pub galaxy_algo: i32,
    pub galaxy_seed: i32,
    pub star_count: i32,
    pub resource_multiplier: f32,
}

impl GameDesc {
    pub fn new(seed: i32) -> Self {
        Self {
            galaxy_algo: 20200403,
            galaxy_seed: seed,
            star_count: 64,
            resource_multiplier: 1.0,
        }
    }

    pub fn is_infinite_resource(&self) -> bool {
        self.resource_multiplier >= 99.5
    }

    pub fn is_rare_resource(&self) -> bool {
        self.resource_multiplier <= 0.100100003182888
    }

    pub fn oil_amount_multipler(&self) -> f32 {
        if self.is_rare_resource() {
            0.5
        } else {
            1.0
        }
    }

    pub fn gas_coef(&self) -> f32 {
        if self.is_rare_resource() {
            0.8
        } else {
            1.0
        }
    }
}
