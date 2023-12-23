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
