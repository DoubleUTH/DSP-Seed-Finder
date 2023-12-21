#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpectrType {
    M,
    K,
    G,
    F,
    A,
    B,
    O,
    X,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
