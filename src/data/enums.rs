use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum StarType {
    MainSeqStar,
    GiantStar,
    WhiteDwarf,
    NeutronStar,
    BlackHole,
}

impl Default for StarType {
    fn default() -> Self {
        Self::MainSeqStar
    }
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Copy, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum SpectrType {
    M = -4,
    K = -3,
    G = -2,
    F = -1,
    A = 0,
    B = 1,
    O = 2,
    X = 3,
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum PlanetType {
    None,
    Vocano,
    Ocean,
    Desert,
    Ice,
    Gas,
}

impl Default for PlanetType {
    fn default() -> Self {
        Self::None
    }
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum ThemeDistribute {
    Default,
    Birth,
    Interstellar,
    Rare,
}

impl Default for ThemeDistribute {
    fn default() -> Self {
        Self::Default
    }
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum VeinType {
    None,
    Iron,
    Copper,
    Silicium,
    Titanium,
    Stone,
    Coal,
    Oil,
    Fireice,
    Diamond,
    Fractal,
    Crysrub,
    Grat,
    Bamboo,
    Mag,
    Max,
}

impl Default for VeinType {
    fn default() -> Self {
        Self::None
    }
}

impl VeinType {
    pub fn is_rare(&self) -> bool {
        matches!(
            self,
            VeinType::Fireice
                | VeinType::Diamond
                | VeinType::Fractal
                | VeinType::Crysrub
                | VeinType::Grat
                | VeinType::Bamboo
        )
    }
}
