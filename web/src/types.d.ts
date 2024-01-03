import type {
    SpectrType,
    StarType as EStarType,
    PlanetType,
    VeinType,
    ConditionType,
    RuleType,
    GasType,
    OceanType as EOceanType,
} from "./enums"

declare global {
    declare type integer = number
    declare type float = number

    declare interface GameDesc {
        seed: integer
        starCount?: integer
        resourceMultiplier?: float
    }

    declare interface Galaxy {
        seed: integer
        stars: Star[]
    }

    declare type Position = [x: float, y: float, z: float]

    declare interface Star {
        index: integer
        position: Position
        name: string
        mass: float
        lifetime: float
        age: float
        temperature: float
        type: EStarType
        spectr: SpectrType
        luminosity: float
        radius: float
        dysonRadius: float
        planets: Planet[]
    }

    declare type Gas = [itemId: GasType, rate: float]

    declare interface Planet {
        index: integer
        orbitAround: integer | null
        orbitIndex: integer
        orbitRadius: float
        orbitInclination: float
        orbitLongitude: float
        orbitalPeriod: float
        orbitPhase: float
        obliquity: float
        rotationPeriod: float
        rotationPhase: float
        sunDistance: float
        type: PlanetType
        luminosity: float
        theme: ThemeProto
        veins: Vein[]
        gases: Gas[]
    }

    declare interface ThemeProto {
        id: integer
        name: string
        waterItemId: EOceanType
        wind: float
    }

    declare interface Vein {
        veinType: VeinType
        minGroup: integer
        maxGroup: integer
        minPatch: integer
        maxPatch: integer
        minAmount: integer
        maxAmount: integer
    }

    declare namespace Condition {
        export type Eq = { type: ConditionType.Eq; value: float }
        export type Neq = { type: ConditionType.Neq; value: float }
        export type Lt = { type: ConditionType.Lt; value: float }
        export type Lte = { type: ConditionType.Lte; value: float }
        export type Gt = { type: ConditionType.Gt; value: float }
        export type Gte = { type: ConditionType.Gte; value: float }
        export type Between = {
            type: ConditionType.Between
            value: [float, float]
        }
        export type NotBetween = {
            type: ConditionType.NotBetween
            value: [float, float]
        }
    }

    declare type Condition =
        | Condition.Eq
        | Condition.Neq
        | Condition.Lt
        | Condition.Lte
        | Condition.Gt
        | Condition.Gte

    declare namespace Rule {
        export type None = { type: RuleType.None }
        export type And = { type: RuleType.And; rules: Rule[] }
        export type Or = { type: RuleType.Or; rules: Rule[] }
        export type Luminosity = {
            type: RuleType.Luminosity
            condition: Condition
        }
        export type DysonRadius = {
            type: RuleType.DysonRadius
            condition: Condition
        }
        export type AverageVeinAmount = {
            type: RuleType.AverageVeinAmount
            vein: VeinType
            condition: Condition
        }
        export type Spectr = {
            type: RuleType.Spectr
            spectr: SpectrType[]
        }
        export type TidalLockCount = {
            type: RuleType.TidalLockCount
            condition: Condition
        }
        export type OceanType = {
            type: RuleType.OceanType
            oceanType: EOceanType
        }
        export type StarType = {
            type: RuleType.StarType
            starType: EStarType[]
        }
        export type GasCount = {
            type: RuleType.GasCount
            ice: boolean | null
            condition: Condition
        }
        export type SatelliteCount = {
            type: RuleType.SatelliteCount
            condition: Condition
        }
        export type Birth = {
            type: RuleType.Birth
        }
        export type ThemeId = {
            type: RuleType.ThemeId
            themeIds: integer[]
        }
        export type PlanetCount = {
            type: RuleType.PlanetCount
            condition: Condition
        }
        export type BirthDistance = {
            type: RuleType.BirthDistance
            condition: Condition
        }
        export type XDistance = {
            type: RuleType.XDistance
            condition: Condition
        }
        export type GasRate = {
            type: RuleType.GasRate
            gasType: GasType
            condition: Condition
        }
    }

    declare type SimpleRule =
        | Rule.None
        | Rule.Luminosity
        | Rule.DysonRadius
        | Rule.AverageVeinAmount
        | Rule.Spectr
        | Rule.TidalLockCount
        | Rule.OceanType
        | Rule.StarType
        | Rule.GasCount
        | Rule.SatelliteCount
        | Rule.Birth
        | Rule.ThemeId
        | Rule.PlanetCount
        | Rule.BirthDistance
        | Rule.XDistance
        | Rule.GasRate

    declare type CompoundRule = Rule.And | Rule.Or

    declare type Rule = SimpleRule | CompoundRule

    declare interface FindOptions {
        gameDesc: Omit<GameDesc, "seed">
        range: [number, number]
        rule: Rule
        concurrency: integer
        autosave: integer
        onError?: (error?: any) => void
        onResult?: (result: FindResult) => void
        onProgress?: (current: number) => void
        onComplete?: () => void
        onInterrupt?: () => void
    }

    declare interface WorldGen {
        generate(gameDesc: GameDesc): Promise<Galaxy>
        find(options: FindOptions): void
        stop(): void
    }

    declare interface FindResult {
        seed: integer
        indexes: integer[]
    }

    declare interface Store {
        settings: Settings
    }

    declare interface Settings {
        darkMode: boolean
        view: {
            starCount: integer
            resourceMultipler: float
        }
    }

    declare interface ProfileInfo {
        id: string
        name: string
        createdAt: integer
    }

    declare interface ProfileProgress {
        id: string
        starCount: integer
        resourceMultiplier: float
        autosave: float
        concurrency: integer
        start: integer
        end: integer
        current: integer
        found: integer
        rules: SimpleRule[][]
    }
    declare interface ProgressResult {
        id: integer
        seed: integer
        index: integer
    }
}
