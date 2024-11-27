import {
    CompositeRuleType,
    ConditionType,
    GasType,
    RuleType,
    SpectrType,
    StarType,
    VeinType,
} from "./enums"

export function toPrecision(number: number, precision: number) {
    return number.toLocaleString([], {
        minimumFractionDigits: 0,
        maximumFractionDigits: precision,
    })
}

export function formatNumber(number: number, precision: number): string {
    if (number >= 1e6) {
        return toPrecision(number / 1e6, precision) + "M"
    } else if (number >= 1e4) {
        return toPrecision(number / 1e3, precision) + "K"
    } else {
        return toPrecision(number, precision)
    }
}

function modifyCondition(condition: Condition, fn: (value: float) => float) {
    return {
        ...condition,
        value: fn(condition.value),
    }
}

function fixRule(rule: SimpleRule): SimpleRule {
    if (
        rule.type === RuleType.AverageVeinAmount &&
        rule.vein === VeinType.Oil
    ) {
        return {
            ...rule,
            condition: modifyCondition(rule.condition, (value) => value * 25e3),
        }
    }
    if (rule.type === RuleType.XDistance) {
        return {
            ...rule,
            all: !!rule.all,
        }
    }
    return rule
}

export function constructRule(rules: SimpleRule[][]): Rule {
    const rs: Rule[] = rules.map((r) =>
        r.length === 1
            ? fixRule(r[0]!)
            : { type: RuleType.Or, rules: r.map(fixRule) },
    )
    return rs.length === 1 ? rs[0]! : { type: RuleType.And, rules: rs }
}

export function constructMultiRule(multiRules: MultiRule[][]): CompositeRule {
    const raw: CompositeRule[] = multiRules.map((rs): CompositeRule => {
        const x = rs.map(
            ({ condition, rules }): Rule.Composite => ({
                type: CompositeRuleType.Composite,
                condition,
                rule: constructRule(rules),
            }),
        )
        return x.length === 1
            ? x[0]!
            : { type: CompositeRuleType.CompositeOr, rules: x }
    })
    return raw.length === 1
        ? raw[0]!
        : { type: CompositeRuleType.CompositeAnd, rules: raw }
}

export const minStarCount = 32
export const maxStarCount = 64
export const defaultStarCount = 64
export const metersPerAU = 40000

export const resourceMultiplers: ReadonlyArray<float> = [
    0.1, 0.5, 0.8, 1, 1.5, 2, 3, 5, 8, 100,
]
export const defaultResourceMultipler = 1

export const veinNames: Record<VeinType, string> = {
    [VeinType.None]: "",
    [VeinType.Iron]: "Iron Ore",
    [VeinType.Copper]: "Copper Ore",
    [VeinType.Silicium]: "Silicon Ore",
    [VeinType.Titanium]: "Titanium Ore",
    [VeinType.Stone]: "Stone",
    [VeinType.Coal]: "Coal",
    [VeinType.Oil]: "Crude Oil",
    [VeinType.Fireice]: "Fire Ice",
    [VeinType.Diamond]: "Kimberlite Ore",
    [VeinType.Fractal]: "Fractal Silicon",
    [VeinType.Crysrub]: "Organic Crystal",
    [VeinType.Grat]: "Grating Crystal",
    [VeinType.Bamboo]: "Stalagmite Crystal",
    [VeinType.Mag]: "Unipolar Magnet",
}

export const gasNames: Record<GasType, string> = {
    [GasType.None]: "",
    [GasType.Fireice]: "Fire Ice",
    [GasType.Hydrogen]: "Hydrogen",
    [GasType.Deuterium]: "Deuterium",
}

export const veinOrder: VeinType[] = [
    VeinType.Iron,
    VeinType.Copper,
    VeinType.Silicium,
    VeinType.Titanium,
    VeinType.Stone,
    VeinType.Coal,
    VeinType.Oil,
    VeinType.Fireice,
    VeinType.Diamond,
    VeinType.Fractal,
    VeinType.Crysrub,
    VeinType.Grat,
    VeinType.Bamboo,
    VeinType.Mag,
]

export const gasOrder: GasType[] = [
    GasType.Fireice,
    GasType.Hydrogen,
    GasType.Deuterium,
]

export function statVein(vein: Vein): VeinStat {
    const min = vein.minGroup * vein.minPatch * vein.minAmount
    const max = vein.maxGroup * vein.maxPatch * vein.maxAmount
    const avg =
        ((vein.minGroup + vein.maxGroup) *
            (vein.minPatch + vein.maxPatch) *
            (vein.minAmount + vein.maxAmount)) /
        8
    return { veinType: vein.veinType, min, max, avg }
}

export function getSearch({
    count,
    multipler,
}: {
    count: integer
    multipler: float
}) {
    const params = new URLSearchParams()
    if (count !== defaultStarCount) {
        params.set("count", String(count))
    }
    if (multipler !== defaultResourceMultipler) {
        params.set("multipler", String(multipler))
    }
    const str = params.toString()
    return str ? "?" + str : ""
}

export const romans = ["I", "II", "III", "IV", "V", "VI"]

export const planetTypes: Record<number, string> = {
    1: "Mariterra",
    2: "Gas Giant",
    3: "Gas Giant",
    4: "Ice Giant",
    5: "Ice Giant",
    6: "Scorchedia",
    7: "Geloterra",
    8: "Tropicana",
    9: "Lava",
    10: "Glacieon",
    11: "Desolus",
    12: "Gobi",
    13: "Sulfuria",
    14: "Crimsonis",
    15: "Prairiea",
    16: "Aquatica",
    17: "Halitum",
    18: "Sakura Ocean",
    19: "Cyclonius",
    20: "Maroonfrost",
    21: "Gas Giant",
    22: "Savanna",
    23: "Onyxtopia",
    24: "Icefrostia",
    25: "Pandora Swamp",
}

export const conditionTypeNames: Record<ConditionType, string> = {
    [ConditionType.Eq]: "exactly",
    [ConditionType.Neq]: "not equal to",
    [ConditionType.Gt]: "greater than",
    [ConditionType.Gte]: "at least",
    [ConditionType.Lt]: "less than",
    [ConditionType.Lte]: "at most",
}

export function validateRules(rules: SimpleRule[][]): boolean {
    if (rules.length === 0) return false
    for (const group of rules) {
        if (group.length === 0) return false
        for (const rule of group) {
            if (rule.type === RuleType.None) return false
            // TODO: Validate individual rule here
        }
    }
    return true
}

export function validateMultiRule(rules: MultiRule[][]): boolean {
    return (
        rules.length > 0 &&
        rules.every(
            (rs) => rs.length > 0 && rs.every((r) => validateRules(r.rules)),
        )
    )
}

export function getStarType(star: Star) {
    if (star.type === StarType.GiantStar) {
        switch (star.spectr) {
            case SpectrType.M:
            case SpectrType.K:
                return "Red Giant"
            case SpectrType.G:
            case SpectrType.F:
                return "Yellow Giant"
            case SpectrType.A:
                return "White Giant"
            default:
                return "Blue Giant"
        }
    } else if (star.type === StarType.WhiteDwarf) {
        return "White Dwarf"
    } else if (star.type === StarType.NeutronStar) {
        return "Neutron Star"
    } else if (star.type === StarType.BlackHole) {
        return "Black Hole"
    } else {
        return star.spectr + " type Star"
    }
}

export function distanceFromBirth([x, y, z]: Position): float {
    return Math.sqrt(x * x + y * y + z * z)
}

export function distanceFrom(
    [x1, y1, z1]: Position,
    [x2, y2, z2]: Position,
): float {
    const dx = x1 - x2
    const dy = y1 - y2
    const dz = z1 - z2
    return Math.sqrt(dx * dx + dy * dy + dz * dz)
}

export function nearestDistanceFrom(
    reference: Position,
    positions: Position[],
): float {
    return positions
        .map((p) => distanceFrom(reference, p))
        .reduce((acc, val) => (acc < val ? acc : val), Infinity)
}

export function furthestDistanceFrom(
    reference: Position,
    positions: Position[],
): float {
    return positions
        .map((p) => distanceFrom(reference, p))
        .reduce((acc, val) => (acc > val ? acc : val), -Infinity)
}
