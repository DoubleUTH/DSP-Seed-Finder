export enum StarType {
    MainSeqStar = "MainSeqStar",
    GiantStar = "GiantStar",
    WhiteDwarf = "WhiteDwarf",
    NeutronStar = "NeutronStar",
    BlackHole = "BlackHole",
}

export enum SpectrType {
    M = "M",
    K = "K",
    G = "G",
    F = "F",
    A = "A",
    B = "B",
    O = "O",
    X = "X",
}

export enum PlanetType {
    None = "None",
    Vocano = "Vocano",
    Ocean = "Ocean",
    Desert = "Desert",
    Ice = "Ice",
    Gas = "Gas",
}

export enum VeinType {
    None = "None",
    Iron = "Iron",
    Copper = "Copper",
    Silicium = "Silicium",
    Titanium = "Titanium",
    Stone = "Stone",
    Coal = "Coal",
    Oil = "Oil",
    Fireice = "Fireice",
    Diamond = "Diamond",
    Fractal = "Fractal",
    Crysrub = "Crysrub",
    Grat = "Grat",
    Bamboo = "Bamboo",
    Mag = "Mag",
}

export enum RuleType {
    None = "None",
    And = "And",
    Or = "Or",
    Birth = "Birth", // 10
    StarType = "StarType", // 11
    BirthDistance = "BirthDistance", // 12
    XDistance = "XDistance", // 13
    Luminosity = "Luminosity", // 20
    Spectr = "Spectr", // 21
    DysonRadius = "DysonRadius", // 22
    PlanetCount = "PlanetCount", // 30
    SatelliteCount = "SatelliteCount", // 31
    TidalLockCount = "TidalLockCount", // 33
    PlanetInDysonCount = "PlanetInDysonCount", // 34
    ThemeId = "ThemeId", // 40
    GasCount = "GasCount", // 41 / 32
    OceanType = "OceanType", // 42
    GasRate = "GasRate", // 50
    AverageVeinAmount = "AverageVeinAmount", // 51
}

export enum ConditionType {
    Eq = "Eq",
    Neq = "Neq",
    Lt = "Lt",
    Lte = "Lte",
    Gt = "Gt",
    Gte = "Gte",
}

export enum GasType {
    None = 0,
    Fireice = 1011,
    Hydrogen = 1120,
    Deuterium = 1121,
}

export enum OceanType {
    None = 0,
    Ice = -2,
    Lava = -1,
    Water = 1000,
    Sulfur = 1116,
}
