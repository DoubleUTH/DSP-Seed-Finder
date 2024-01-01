import { GasType, RuleType, VeinType } from "./enums"

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

export function constructRule(rules: SimpleRule[][]): Rule {
    const rs: Rule[] = rules.map((r) =>
        r.length === 1 ? r[0]! : { type: RuleType.Or, rules: r },
    )
    return rs.length === 1 ? rs[0]! : { type: RuleType.And, rules: rs }
}

export const minStarCount = 32
export const maxStarCount = 64
export const defaultStarCount = 64

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
