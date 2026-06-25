import type {
    SpectrType,
    StarType as EStarType,
    PlanetType,
    VeinType,
    ConditionType,
    RuleType,
    GasType,
    OceanType as EOceanType,
    CompositeRuleType,
} from "./enums"
import type { ALL_LANGS } from "./constants"

declare global {
    declare type Lang = (typeof ALL_LANGS)[number]
    declare type integer = number
    declare type float = number

    declare interface GameParameters {
        starCount: integer
        resourceMultiplier: float
        hiveInitialColonize: float
        hiveMaxDensity: float
        useActualVeins: boolean
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
        initialHiveCount: integer
        maxHiveCount: integer
        color: float
        planets: Planet[]
    }

    declare type Gas = [itemId: GasType, rate: float]

    declare type PlanetVeins =
        | {
              veins: EstimatedVein[]
          }
        | {
              actualVeins: ActualVein[]
          }

    declare type Planet = {
        index: integer
        orbitAround: integer | null
        orbitIndex: integer
        orbitRadius: float
        orbitInclination: float
        orbitLongitude: float
        orbitalPeriod: float
        obliquity: float
        rotationPeriod: float
        sunDistance: float
        type: PlanetType
        luminosity: float
        theme: ThemeProto
        gases: Gas[]
    } & PlanetVeins

    declare interface ThemeProto {
        id: integer
        name: string
        waterItemId: EOceanType
        wind: float
    }

    declare interface EstimatedVein {
        veinType: VeinType
        minGroup: integer
        maxGroup: integer
        minPatch: integer
        maxPatch: integer
        minAmount: integer
        maxAmount: integer
    }

    declare interface ActualVein {
        veinType: VeinType
        amount: integer
    }

    declare interface VeinStat {
        veinType: VeinType
        min: integer
        max: integer
        avg: float
    }

    declare namespace Condition {
        export type Eq = { type: ConditionType.Eq; value: float }
        export type Neq = { type: ConditionType.Neq; value: float }
        export type Lt = { type: ConditionType.Lt; value: float }
        export type Lte = { type: ConditionType.Lte; value: float }
        export type Gt = { type: ConditionType.Gt; value: float }
        export type Gte = { type: ConditionType.Gte; value: float }
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
            type: RuleType.AverageVeinAmount // legacy name
            useActual?: boolean
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
            excludeGiant: boolean
        }
        export type BirthDistance = {
            type: RuleType.BirthDistance
            condition: Condition
        }
        export type XDistance = {
            type: RuleType.XDistance
            all?: boolean
            condition: Condition
        }
        export type SpectrDistance = {
            type: RuleType.SpectrDistance
            spectr: SpectrType
            countCondition: Condition
            distanceCondition: Condition
        }
        export type GasRate = {
            type: RuleType.GasRate
            gasType: GasType
            condition: Condition
        }
        export type PlanetInDysonCount = {
            type: RuleType.PlanetInDysonCount
            includeGiant: boolean
            condition: Condition
        }
        export type HiveCount = {
            type: RuleType.HiveCount
            condition: Condition
            initial: boolean
        }

        export type Composite = {
            type: CompositeRuleType.Composite
            rule: Rule
            condition: Condition
        }

        export type CompositeAnd = {
            type: CompositeRuleType.CompositeAnd
            rules: CompositeRule[]
        }

        export type CompositeOr = {
            type: CompositeRuleType.CompositeOr
            rules: CompositeRule[]
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
        | Rule.SpectrDistance
        | Rule.GasRate
        | Rule.PlanetInDysonCount
        | Rule.HiveCount

    declare type CompoundRule = Rule.And | Rule.Or

    declare type Rule = SimpleRule | CompoundRule

    declare type CompositeRule =
        | Rule.Composite
        | Rule.CompositeAnd
        | Rule.CompositeOr

    declare type FindRange = [integer, integer] | Int32Array<ArrayBuffer>

    declare interface FindOptions {
        gameDesc: GameParameters
        batchSize: integer
        nextBatchId: integer
        range: FindRange
        rule: Rule | CompositeRule
        concurrency: integer
        autosave: integer
        onError: (error?: any) => void
        onResult: (result: integer[]) => void
        onProgress: (nextBatchId: number) => void
        onComplete: () => void
        onInterrupt: () => void
    }

    declare interface InternalFindOptions {
        gameDesc: GameParameters
        batchSize: integer
        nextBatchId: integer
        range: FindRange
        rule: Rule | CompositeRule
        concurrency: integer
        onBatchResult: (batchId: integer, result: integer[]) => void
        onInterrupt: () => void
    }

    declare interface GenerateDatabaseOptions {
        name: string
        range: [integer, integer]
        params: GameParameters
        concurrency: integer
        onProgress: (seed: integer) => void
        onInterrupt: () => void
        onComplete: () => void
        onError: (error?: any) => void
    }

    declare interface InternalGenerateDatabaseOptions {
        name: string
        range: [integer, integer]
        params: GameParameters
        concurrency: integer
        onProgress: (seed: integer) => void
        onInterrupt: () => void
    }

    declare interface WorldGen {
        generate(seed: integer, gameDesc: GameParameters): Promise<Galaxy>
        find(options: InternalFindOptions): Promise<void>
        createDatabase(options: InternalGenerateDatabaseOptions): Promise<void>
        stop(): void
    }

    declare interface Store {
        settings: Settings
        searching: boolean
    }

    declare interface Settings {
        darkMode: boolean
        language: Lang
        view: GameParameters
    }

    declare interface ProfileInfo {
        id: string
        name: string
        createdAt: integer
    }

    declare interface ProfileProgressInfo {
        id: string
        params: GameParameters
        autosave: float
        concurrency: integer
        range: FindRange
        total: integer
        found: integer
        batchSize: integer
        nextBatchId: integer
    }

    declare interface ProfileProgress extends ProfileProgressInfo {
        rules: SimpleRule[][]
    }

    declare interface MultiProfileProgress extends ProfileProgressInfo {
        multiRules: MultiRule[][]
    }

    declare interface MultiRule {
        name: string
        rules: SimpleRule[][]
        condition: Condition
    }

    declare interface ProgressResult {
        id: integer
        seed: integer
        index: integer
    }

    declare interface MultiProgressResult {
        seed: integer
    }

    declare interface Exporter {
        (options: ExportOptions): Promise<Blob | null>
    }

    declare interface ExportOptions {
        language: Lang
        format: "csv" | "xlsx" | "txt"
        concurrency: integer
        params: GameParameters
        results: integer[]
        onProgress: (current: integer) => boolean
        onGenerate: () => void
    }

    declare interface ExportData {
        seed: integer
        stars: any[]
        planets: any[]
        indexes?: integer[]
    }
}
