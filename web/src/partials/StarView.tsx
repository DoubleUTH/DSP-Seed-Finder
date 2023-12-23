import { createStore } from "solid-js/store"
import { GasType, OceanType, SpectrType, StarType, VeinType } from "../enums"
import { formatNumber, toPrecision } from "../util"
import styles from "./StarView.module.css"
import { Component, Show, For } from "solid-js"
import { AiOutlineDown } from "solid-icons/ai"
import clsx from "clsx"

function distanceFromBirth([x, y, z]: [float, float, float]): float {
    return Math.sqrt(x * x + y * y + z * z)
}

function type(star: Star) {
    if (star.starType === StarType.GiantStar) {
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
    } else if (star.starType === StarType.WhiteDwarf) {
        return "White Dwarf"
    } else if (star.starType === StarType.NeutronStar) {
        return "Neutron Star"
    } else if (star.starType === StarType.BlackHole) {
        return "Black Hole"
    } else {
        return star.spectr + " type Star"
    }
}

const gasOrder: GasType[] = [
    GasType.Fireice,
    GasType.Hydrogen,
    GasType.Deuterium,
]

const veinOrder: VeinType[] = [
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

const veinNames: Record<VeinType, string> = {
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

const gasNames: Record<GasType, string> = {
    [GasType.Fireice]: "Fire Ice",
    [GasType.Hydrogen]: "Hydrogen",
    [GasType.Deuterium]: "Deuterium",
}

type VeinStat = {
    veinType: VeinType
    min: integer
    max: integer
    avg: float
}

function statVein(vein: Vein): VeinStat {
    const min = vein.minGroup * vein.minPatch * vein.minAmount
    const max = vein.maxGroup * vein.maxPatch * vein.maxAmount
    const avg =
        ((vein.minGroup + vein.maxGroup) *
            (vein.minPatch + vein.maxPatch) *
            (vein.minAmount + vein.maxAmount)) /
        8
    return { veinType: vein.veinType, min, max, avg }
}

function combineVeins(star: Star): VeinStat[] {
    const veins: Record<VeinType, VeinStat> = {} as any
    for (const planet of star.planets) {
        for (const vein of planet.veins) {
            const stat = statVein(vein)
            const existing = veins[vein.veinType]
            if (existing) {
                existing.min += stat.min
                existing.max += stat.max
                existing.avg += stat.avg
            } else {
                veins[vein.veinType] = { ...stat }
            }
        }
    }
    return veinOrder.map((type) => veins[type]).filter((x) => x)
}

function combineGases(star: Star): Gas[] {
    const veins: Record<GasType, float> = {} as any
    for (const planet of star.planets) {
        for (const [type, amount] of planet.gases) {
            veins[type] ??= 0
            veins[type] += amount
        }
    }
    return gasOrder
        .filter((type) => veins[type])
        .map((type) => [type, veins[type]])
}

function hasWater(star: Star): boolean {
    return !!star.planets.find(
        (planet) => planet.themeProto.waterItemId === OceanType.Water,
    )
}

function hasSulfur(star: Star): boolean {
    return !!star.planets.find(
        (planet) => planet.themeProto.waterItemId === OceanType.Sulfur,
    )
}

function formatVein(amount: number, isOil: boolean): string {
    if (isOil) {
        return formatNumber(amount * 4e-5, 2) + " /s"
    } else {
        return toPrecision(amount, 0)
    }
}

const Expand: Component<{ expand: boolean; toggle: () => void }> = (props) => (
    <div
        class={clsx(styles.expand, props.expand && styles.expanded)}
        onClick={() => props.toggle()}
    >
        <AiOutlineDown />
    </div>
)

const StarDetail: Component<{ star: Star; expand: boolean }> = (props) => (
    <>
        <div class={styles.row}>
            <div class={styles.field}>Type</div>
            <div class={styles.value}>{type(props.star)}</div>
        </div>
        <div class={styles.row}>
            <div class={styles.field}>Spectral Class</div>
            <div class={styles.value}>{props.star.spectr}</div>
        </div>
        <div class={styles.row}>
            <div class={styles.field}>Luminosity</div>
            <div class={styles.value}>
                {formatNumber(props.star.luminosity, 3)} L
            </div>
        </div>
        <div class={styles.row}>
            <div class={styles.field}>Distance from birth</div>
            <div class={styles.value}>
                {formatNumber(distanceFromBirth(props.star.position), 2)} ly
            </div>
        </div>
        <div class={styles.row}>
            <div class={styles.field}>Max Dyson Sphere Radius</div>
            <div class={styles.value}>
                {toPrecision(Math.round(props.star.dysonRadius * 800) * 100, 0)}{" "}
                m
            </div>
        </div>
        <Show when={props.expand}>
            <div class={styles.row}>
                <div class={styles.field}>Radius</div>
                <div class={styles.value}>
                    {toPrecision(props.star.radius * 1600, 0)} m
                </div>
            </div>
            <div class={styles.row}>
                <div class={styles.field}>Mass</div>
                <div class={styles.value}>
                    {formatNumber(props.star.mass, 3)} M
                </div>
            </div>
            <div class={styles.row}>
                <div class={styles.field}>Temperature</div>
                <div class={styles.value}>
                    {formatNumber(props.star.temperature, 0)} K
                </div>
            </div>
            <div class={styles.row}>
                <div class={styles.field}>Age</div>
                <div class={styles.value}>
                    {formatNumber(
                        props.star.age *
                            props.star.lifetime *
                            0.999999977648258,
                        0,
                    )}{" "}
                    Myrs
                </div>
            </div>
        </Show>
    </>
)

const StarVeins: Component<{ star: Star }> = (props) => (
    <>
        <For each={combineVeins(props.star)}>
            {(vein) => (
                <div class={styles.row}>
                    <div class={styles.field}>{veinNames[vein.veinType]}</div>
                    <div class={clsx(styles.value, styles.estimate)}>
                        {formatVein(vein.avg, vein.veinType === VeinType.Oil)}
                    </div>
                </div>
            )}
        </For>
        <Show when={hasWater(props.star)}>
            <div class={styles.row}>
                <div class={styles.field}>Water</div>
                <div class={styles.value}>Ocean</div>
            </div>
        </Show>
        <Show when={hasSulfur(props.star)}>
            <div class={styles.row}>
                <div class={styles.field}>Sulfuric Acid</div>
                <div class={styles.value}>Ocean</div>
            </div>
        </Show>
        <For each={combineGases(props.star)}>
            {([type, amount]) => (
                <div class={styles.row}>
                    <div class={styles.field}>{gasNames[type]}</div>
                    <div class={styles.value}>{formatNumber(amount, 4)} /s</div>
                </div>
            )}
        </For>
    </>
)

const StarView: Component<{ star: Star; stars?: Star[] }> = (props) => {
    const [expand, setExpand] = createStore({
        detail: false,
        planets: {} as Record<number, boolean>,
    })

    return (
        <div class={styles.view}>
            <div class={styles.card}>
                <Expand
                    expand={expand.detail}
                    toggle={() => setExpand("detail", (x) => !x)}
                />
                <div class={styles.title}>
                    <span>{props.star.name}</span>
                    <span class={styles.index}>#{props.star.index + 1}</span>
                </div>
                <StarDetail star={props.star} expand={expand.detail} />
            </div>
            <div class={styles.card}>
                <div class={styles.title}>
                    <span>Resources</span>
                </div>
                <StarVeins star={props.star} />
            </div>
        </div>
    )
}

export default StarView
