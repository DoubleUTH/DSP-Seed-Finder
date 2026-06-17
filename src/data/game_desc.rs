use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDesc {
    #[serde(default = "GameDesc::default_star_count")]
    pub star_count: usize,
    #[serde(default = "GameDesc::default_resource_multiplier")]
    pub resource_multiplier: f32,
    #[serde(default = "GameDesc::default_hive_initial_colonize")]
    pub hive_initial_colonize: f64,
    #[serde(default = "GameDesc::default_hive_max_density")]
    pub hive_max_density: f64,
    #[serde(default)]
    pub use_actual_veins: bool,
}

impl GameDesc {
    pub fn default_star_count() -> usize {
        64
    }
    pub fn default_resource_multiplier() -> f32 {
        1.0
    }
    pub fn default_hive_initial_colonize() -> f64 {
        1.0
    }
    pub fn default_hive_max_density() -> f64 {
        1.0
    }

    pub fn is_infinite_resource(&self) -> bool {
        self.resource_multiplier >= 99.5
    }

    pub fn is_rare_resource(&self) -> bool {
        self.resource_multiplier <= 0.1001
    }

    pub fn oil_amount_multiplier(&self) -> f32 {
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
