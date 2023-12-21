use super::enums::VeinType;

#[derive(Debug, Clone)]
pub struct Vein {
    pub vein_type: VeinType,
    pub min_group: i32,
    pub max_group: i32,
    pub min_patch: i32,
    pub max_patch: i32,
    pub min_amount: i32,
    pub max_amount: i32,
}

impl Default for Vein {
    fn default() -> Self {
        Self {
            vein_type: VeinType::None,
            min_group: 0,
            max_group: 0,
            min_patch: 0,
            max_patch: 0,
            min_amount: 0,
            max_amount: 0,
        }
    }
}

impl Vein {
    pub fn new() -> Self {
        Default::default()
    }
}
