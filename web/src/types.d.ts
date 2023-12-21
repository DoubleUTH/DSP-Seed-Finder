import type { SpectrType, StarType, PlanetType, VeinType } from "./enums"

declare global {
    declare type integer = number
    declare type float = number

    declare interface GameDesc {
        readonly seed: integer
        readonly starCount?: integer
        readonly resourceMultiplier?: float
    }

    declare interface Galaxy {
        readonly seed: integer
        readonly stars: readonly Star[]
    }

    declare interface Star {
        readonly id: integer
        readonly position: readonly [float, float, float]
        readonly name: string
        readonly mass: float
        readonly lifetime: float
        readonly age: float
        readonly temperature: float
        readonly type: StarType
        readonly spectr: SpectrType
        readonly luminosity: float
        readonly radius: float
        readonly dysonRadius: float
        readonly planets: readonly Planet[]
    }

    declare interface Planet {
        readonly id: integer
        readonly index: integer
        readonly orbitAround: integer
        readonly orbitIndex: integer
        readonly name: string
        readonly isBirth: bool
        readonly orbitRadius: float
        readonly orbitInclination: float
        readonly orbitLongitude: float
        readonly orbitalPeriod: float
        readonly orbitPhase: float
        readonly obliquity: float
        readonly rotationPeriod: float
        readonly rotationPhase: float
        readonly sunDistance: float
        readonly planetType: PlanetType
        readonly habitableBias: float
        readonly temperatureBias: float
        readonly themeProto: ThemeProto
        readonly veins: readonly Vein[]
        readonly gases: readonly (readonly [itemId: integer, rate: float])[]
    }

    declare interface ThemeProto {
        readonly id: integer
        readonly name: string
    }

    declare interface Vein {
        readonly veinType: VeinType
        readonly minGroup: integer
        readonly maxGroup: integer
        readonly minPatch: integer
        readonly maxPatch: integer
        readonly minAmount: integer
        readonly maxAmount: integer
    }
}
