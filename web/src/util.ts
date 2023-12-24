import { RuleType } from "./enums"

export function toPrecision(number: number, precision: number) {
    return number.toLocaleString([], {
        minimumFractionDigits: precision,
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
