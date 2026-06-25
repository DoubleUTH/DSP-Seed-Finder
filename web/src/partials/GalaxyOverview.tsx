import styles from "~styles"
import Starmap from "./Starmap"
import { Component, createMemo, For, Show } from "solid-js"
import { useLingui } from "#lingui"
import { useStarTypeFullName, useVeinNames, useGasTypeNames } from "../names"
import {
    statVein,
    veinOrder,
    gasOrder,
    formatNumber,
    toPrecision,
} from "../util"
import { VeinType, GasType, StarType, SpectrType } from "../enums"
import Tooltip from "../components/Tooltip"

function combineAllVeins(stars: Star[]): VeinStat[] {
    const veins: Record<VeinType, VeinStat> = {} as any
    for (const star of stars) {
        for (const planet of star.planets) {
            if ("veins" in planet) {
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
            } else {
                for (const vein of planet.actualVeins) {
                    const existing = veins[vein.veinType]
                    if (existing) {
                        existing.avg += vein.amount
                    } else {
                        veins[vein.veinType] = {
                            veinType: vein.veinType,
                            min: 0,
                            max: 0,
                            avg: vein.amount,
                        }
                    }
                }
            }
        }
    }
    return veinOrder.map((type) => veins[type]).filter((x) => x)
}

function combineAllGases(stars: Star[]): Gas[] {
    const gases: Record<GasType, float> = {} as any
    for (const star of stars) {
        for (const planet of star.planets) {
            for (const [type, amount] of planet.gases) {
                gases[type] = (gases[type] ?? 0) + amount
            }
        }
    }
    return gasOrder
        .filter((type) => gases[type])
        .map((type) => [type, gases[type]])
}

function formatVein(amount: number, isOil: boolean): string {
    if (isOil) {
        return formatNumber(amount * 4e-5, 2) + " /s"
    } else {
        return toPrecision(amount, 0)
    }
}

const Vein: Component<{
    stat: VeinStat
    class?: string
}> = (props) => {
    const isOil = () => props.stat.veinType === VeinType.Oil
    const avg = () => formatVein(props.stat.avg, isOil())
    const min = () => formatVein(props.stat.min, isOil())
    const max = () => formatVein(props.stat.max, isOil())
    const { t } = useLingui()
    return (
        <div class={props.class}>
            <Show when={props.stat.min !== props.stat.max} fallback={avg()}>
                ~{" "}
                <Tooltip text={t`Estimated:\n${min()} - ${max()}`}>
                    {avg()}
                </Tooltip>
            </Show>
        </div>
    )
}

const GalaxyOverview: Component<{ galaxy: Galaxy; search: string }> = (
    props,
) => {
    const { t } = useLingui()
    const getStarType = useStarTypeFullName()
    const veinNames = useVeinNames()
    const gasTypeNames = useGasTypeNames()

    const starTypeCounts = createMemo(() => {
        const order = [
            getStarType({ type: StarType.MainSeqStar, spectr: SpectrType.M }),
            getStarType({ type: StarType.MainSeqStar, spectr: SpectrType.K }),
            getStarType({ type: StarType.MainSeqStar, spectr: SpectrType.G }),
            getStarType({ type: StarType.MainSeqStar, spectr: SpectrType.F }),
            getStarType({ type: StarType.MainSeqStar, spectr: SpectrType.A }),
            getStarType({ type: StarType.MainSeqStar, spectr: SpectrType.B }),
            getStarType({ type: StarType.MainSeqStar, spectr: SpectrType.O }),
            getStarType({ type: StarType.GiantStar, spectr: SpectrType.M }),
            getStarType({ type: StarType.GiantStar, spectr: SpectrType.G }),
            getStarType({ type: StarType.GiantStar, spectr: SpectrType.A }),
            getStarType({ type: StarType.GiantStar, spectr: SpectrType.B }),
            getStarType({ type: StarType.WhiteDwarf, spectr: SpectrType.X }),
            getStarType({ type: StarType.NeutronStar, spectr: SpectrType.X }),
            getStarType({ type: StarType.BlackHole, spectr: SpectrType.X }),
        ]
        const counts: Record<string, number> = {}
        for (const star of props.galaxy.stars) {
            const name = getStarType(star)
            counts[name] = (counts[name] ?? 0) + 1
        }
        return order
            .filter((name) => counts[name])
            .map((name) => [name, counts[name]!] as const)
    })

    const allVeins = createMemo(() => combineAllVeins(props.galaxy.stars))
    const allGases = createMemo(() => combineAllGases(props.galaxy.stars))

    return (
        <div class={styles.root}>
            <div class={styles.map}>
                <Starmap galaxy={props.galaxy} search={props.search} />
            </div>
            <div class={styles.info}>
                <div class={styles.card}>
                    <div class={styles.title}>
                        <span>
                            {t`Seed`}: {props.galaxy.seed}
                        </span>
                    </div>
                </div>
                <Show when={starTypeCounts().length > 0}>
                    <div class={styles.card}>
                        <div class={styles.title}>
                            <span>{t`Star types`}</span>
                        </div>
                        <For each={starTypeCounts()}>
                            {([name, count]) => (
                                <div class={styles.row}>
                                    <div class={styles.field}>{name}:</div>
                                    <div class={styles.value}>{count}</div>
                                </div>
                            )}
                        </For>
                    </div>
                </Show>
                <Show when={allVeins().length > 0}>
                    <div class={styles.card}>
                        <div class={styles.title}>
                            <span>{t`Resources`}</span>
                        </div>
                        <For each={allVeins()}>
                            {(vein) => (
                                <div class={styles.row}>
                                    <div class={styles.field}>
                                        {veinNames[vein.veinType]()}:
                                    </div>
                                    <Vein class={styles.value} stat={vein} />
                                </div>
                            )}
                        </For>
                    </div>
                </Show>
                <Show when={allGases().length > 0}>
                    <div class={styles.card}>
                        <div class={styles.title}>
                            <span>{t`Gas rate`}</span>
                        </div>
                        <For each={allGases()}>
                            {([type, amount]) => (
                                <div class={styles.row}>
                                    <div class={styles.field}>
                                        {gasTypeNames[type]()}:
                                    </div>
                                    <div class={styles.value}>
                                        {formatNumber(amount, 4)} /s
                                    </div>
                                </div>
                            )}
                        </For>
                    </div>
                </Show>
            </div>
        </div>
    )
}

export default GalaxyOverview
