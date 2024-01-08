import { gasNames, gasOrder, veinNames, veinOrder } from "../util"

export enum StarField {
    Seed = "Seed",
    Index = "Index",
    Name = "Name",
    PositionX = "X",
    PositionY = "Y",
    PositionZ = "Z",
    Mass = "Mass",
    Age = "Age",
    Temperature = "Temperature",
    Type = "Type",
    Spectr = "Spectral Class",
    Luminosity = "Luminosity",
    Radius = "Radius",
    DysonRadius = "Max Dyson Sphere Radius",
    DistanceFromBirth = "Distance From Start",
    DistanceFromNearestX = "Distance From Nearest X Star",
    DistanceFromFurthestX = "Distance From Furthest X Star",
}

export const starFieldsOrder = [
    StarField.Seed,
    StarField.Index,
    StarField.Name,
    StarField.Type,
    StarField.Spectr,
    StarField.Luminosity,
    StarField.PositionX,
    StarField.PositionY,
    StarField.PositionZ,
    StarField.DistanceFromBirth,
    StarField.DistanceFromNearestX,
    StarField.DistanceFromFurthestX,
    StarField.Radius,
    StarField.DysonRadius,
    StarField.Mass,
    StarField.Age,
    StarField.Temperature,
]

export enum PlanetField {
    Seed = "Seed",
    Index = "Index",
    Name = "Name",
    Theme = "Theme",
    Orbiting = "Orbiting",
    TidallyLocked = "Tidally Locked",
    OrbitRadius = "Orbit Radius",
    OrbitInclination = "Orbit Inclination",
    OrbitLongitude = "Orbit Longitude",
    OrbitalPeriod = "Orbital Period",
    OrbitPhase = "Orbit Phase",
    Obliquity = "Obliquity",
    RotationPeriod = "Rotation Period",
    RotationPhase = "Rotation Phase",
    Wind = "Wind Power",
    Luminosity = "Solar Power",
}

export const planetFieldsOrder = [
    PlanetField.Seed,
    PlanetField.Index,
    PlanetField.Name,
    PlanetField.Theme,
    PlanetField.Orbiting,
    PlanetField.TidallyLocked,
    PlanetField.Wind,
    PlanetField.Luminosity,
    PlanetField.OrbitRadius,
    PlanetField.OrbitInclination,
    PlanetField.OrbitLongitude,
    PlanetField.OrbitalPeriod,
    PlanetField.OrbitPhase,
    PlanetField.Obliquity,
    PlanetField.RotationPeriod,
    PlanetField.RotationPhase,
]

export const veinFieldsOrder = [
    ...veinOrder.flatMap((type) => {
        const name = veinNames[type]
        return [`${name} (Avg)`, `${name} (Min)`, `${name} (Max)`]
    }),
    "Water",
    "Sulfuric Acid",
    ...gasOrder.map((type) => gasNames[type]),
]
