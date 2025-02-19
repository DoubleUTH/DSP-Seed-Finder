import { createStore } from "solid-js/store"
import { GasType, OceanType, PlanetType, StarType, VeinType } from "../enums"
import {
    distanceFromBirth,
    formatNumber,
    furthestDistanceFrom,
    gasNames,
    gasOrder,
    getStarType,
    metersPerAU,
    nearestDistanceFrom,
    planetTypes,
    romans,
    statVein,
    toPrecision,
    veinNames,
    veinOrder,
} from "../util"
import styles from "./StarView.module.css"
import { Component, Show, For, createMemo } from "solid-js"
import { IoChevronDown } from "solid-icons/io"
import clsx from "clsx"
import { A } from "@solidjs/router"
import Tooltip from "../components/Tooltip"

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
        (planet) => planet.theme.waterItemId === OceanType.Water,
    )
}

function hasSulfur(star: Star): boolean {
    return !!star.planets.find(
        (planet) => planet.theme.waterItemId === OceanType.Sulfur,
    )
}

function planetVeins(planet: Planet): VeinStat[] {
    const veins: Record<VeinType, VeinStat> = {} as any
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
    return veinOrder.map((type) => veins[type]).filter((x) => x)
}

function planetGases(planet: Planet): Gas[] {
    const veins: Record<GasType, float> = {} as any
    for (const [type, amount] of planet.gases) {
        veins[type] ??= 0
        veins[type] += amount
    }
    return gasOrder
        .filter((type) => veins[type])
        .map((type) => [type, veins[type]])
}

function formatVein(amount: number, isOil: boolean): string {
    if (isOil) {
        return formatNumber(amount * 4e-5, 2) + " /s"
    } else {
        return toPrecision(amount, 0)
    }
}

function nearbyStars(
    star: Star,
    stars: Star[],
): { star: Star; distance: float }[] {
    const [x1, y1, z1] = star.position
    const result = stars
        .filter((s) => s.index !== star.index)
        .map((s) => {
            const [x2, y2, z2] = s.position
            const dx = x1 - x2
            const dy = y1 - y2
            const dz = z1 - z2
            return { star: s, distance: Math.sqrt(dx * dx + dy * dy + dz * dz) }
        })

    result.sort((a, b) => a.distance - b.distance)

    return result
}

const Expand: Component<{ expand: boolean; toggle: () => void }> = (props) => (
    <div
        class={clsx(styles.expand, props.expand && styles.expanded)}
        onClick={() => props.toggle()}
    >
        <IoChevronDown />
    </div>
)

const StarDetail: Component<{
    star: Star
    expand: boolean
    positions?: Position[]
}> = (props) => (
    <>
        <div class={styles.row}>
            <div class={styles.field}>Type</div>
            <div class={styles.value}>{getStarType(props.star)}</div>
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
            <div class={styles.field}>Distance from Start</div>
            <div class={styles.value}>
                {formatNumber(distanceFromBirth(props.star.position), 1)} ly
            </div>
        </div>
        <Show when={props.positions}>
            <div class={styles.row}>
                <div class={styles.field}>
                    Distance from nearest{" "}
                    <Tooltip text="Black Hole / Neutron Star">X star</Tooltip>
                </div>
                <div class={styles.value}>
                    {formatNumber(
                        nearestDistanceFrom(
                            props.star.position,
                            props.positions!,
                        ),
                        1,
                    )}{" "}
                    ly
                </div>
            </div>
        </Show>
        <Show when={props.positions}>
            <div class={styles.row}>
                <div class={styles.field}>
                    Distance from furthest{" "}
                    <Tooltip text="Black Hole / Neutron Star">X star</Tooltip>
                </div>
                <div class={styles.value}>
                    {formatNumber(
                        furthestDistanceFrom(
                            props.star.position,
                            props.positions!,
                        ),
                        1,
                    )}{" "}
                    ly
                </div>
            </div>
        </Show>
        <div class={styles.row}>
            <div class={styles.field}>Max Dyson Sphere Radius</div>
            <div class={styles.value}>
                {toPrecision(props.star.dysonRadius, 0)} m
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
                    {toPrecision(props.star.temperature, 0)} K
                </div>
            </div>
            <div class={styles.row}>
                <div class={styles.field}>Age</div>
                <div class={styles.value}>
                    {toPrecision(props.star.age * props.star.lifetime, 0)} Myrs
                </div>
            </div>
        </Show>
    </>
)

const Vein: Component<{
    stat: VeinStat
    class?: string
}> = (props) => {
    const isOil = () => props.stat.veinType === VeinType.Oil
    const avg = () => formatVein(props.stat.avg, isOil())
    const min = () => formatVein(props.stat.min, isOil())
    const max = () => formatVein(props.stat.max, isOil())
    return (
        <div class={props.class}>
            ~{" "}
            <Tooltip text={`Estimated:\n${min()} - ${max()}`}>{avg()}</Tooltip>
        </div>
    )
}

const StarVeins: Component<{ star: Star }> = (props) => (
    <>
        <For each={combineVeins(props.star)}>
            {(vein) => (
                <div class={styles.row}>
                    <div class={styles.field}>{veinNames[vein.veinType]}</div>
                    <Vein class={styles.value} stat={vein} />
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

const NearbyStar: Component<{
    seed: integer
    star: Star
    distance: float
    url: string
    newPage?: boolean
}> = (props) => (
    <A
        href={props.url}
        target={props.newPage ? "_blank" : undefined}
        class={clsx(styles.row, styles.nearbyRow)}
    >
        <div>
            <span>{props.star.name}</span>
            <span class={styles.index}>#{props.star.index + 1}</span>
        </div>
        <div>
            <span class={styles.nearbyType}>{getStarType(props.star)}</span>
            <span class={styles.nearbyDistance}>
                {formatNumber(props.distance, 1)} ly
            </span>
        </div>
    </A>
)

const PlanetView: Component<{ star: Star; planet: Planet }> = (props) => {
    function isGas() {
        return props.planet.type === PlanetType.Gas
    }

    return (
        <div class={styles.planet}>
            <div class={styles.planetName}>
                {props.star.name} {romans[props.planet.index]}
            </div>
            <Show when={isGas()}>
                <div class={styles.row}>
                    <div class={styles.field}>Type</div>
                    <div class={styles.value}>
                        {props.planet.gases.find(
                            ([g]) => g === GasType.Deuterium,
                        )
                            ? "Gas Giant"
                            : "Ice Giant"}
                    </div>
                </div>
            </Show>
            <Show when={!isGas()}>
                <Show when={props.planet.orbitAround != null}>
                    <div class={styles.row}>Satellite</div>
                </Show>
                <Show
                    when={
                        props.planet.orbitalPeriod ===
                        props.planet.rotationPeriod
                    }
                >
                    <div class={styles.row}>Tidal locking</div>
                </Show>
                <Show
                    when={
                        props.planet.orbitalPeriod * 0.5 ===
                        props.planet.rotationPeriod
                    }
                >
                    <div class={styles.row}>Orbital Resonance 1 : 2</div>
                </Show>
                <Show
                    when={
                        props.planet.orbitalPeriod * 0.25 ===
                        props.planet.rotationPeriod
                    }
                >
                    <div class={styles.row}>Orbital Resonance 1 : 4</div>
                </Show>
                <Show when={Math.abs(props.planet.obliquity) > 70}>
                    <div class={styles.row}>Horizontal Rotation</div>
                </Show>
                <Show when={props.planet.orbitAround == null}>
                    <div class={styles.row}>
                        <div class={styles.field}>Orbit Radius</div>
                        <div class={styles.value}>
                            {toPrecision(
                                props.planet.orbitRadius * metersPerAU,
                                0,
                            )}{" "}
                            m
                        </div>
                    </div>
                </Show>
                <div class={styles.row}>
                    <div class={styles.field}>Wind Power</div>
                    <div class={styles.value}>
                        {toPrecision(props.planet.theme.wind * 100, 0)}%
                    </div>
                </div>
                <div class={styles.row}>
                    <div class={styles.field}>Solar Power</div>
                    <div class={styles.value}>
                        {toPrecision(props.planet.luminosity * 100, 0)}%
                    </div>
                </div>
                <div class={styles.row}>
                    <div class={styles.field}>Type</div>
                    <div class={styles.value}>
                        {planetTypes[props.planet.theme.id] ||
                            props.planet.theme.id}
                    </div>
                </div>
            </Show>
            <For each={planetVeins(props.planet)}>
                {(vein) => (
                    <div class={styles.row}>
                        <div class={styles.field}>
                            {veinNames[vein.veinType]}
                        </div>
                        <Vein class={styles.value} stat={vein} />
                    </div>
                )}
            </For>
            <Show when={props.planet.theme.waterItemId === OceanType.Water}>
                <div class={styles.row}>
                    <div class={styles.field}>Water</div>
                    <div class={styles.value}>Ocean</div>
                </div>
            </Show>
            <Show when={props.planet.theme.waterItemId === OceanType.Sulfur}>
                <div class={styles.row}>
                    <div class={styles.field}>Sulfuric Acid</div>
                    <div class={styles.value}>Ocean</div>
                </div>
            </Show>
            <For each={planetGases(props.planet)}>
                {([type, amount]) => (
                    <div class={styles.row}>
                        <div class={styles.field}>{gasNames[type]}</div>
                        <div class={styles.value}>
                            {formatNumber(amount, 4)} /s
                        </div>
                    </div>
                )}
            </For>
        </div>
    )
}

const StarView: Component<{
    star: Star
    galaxy?: Galaxy
    buildUrl: (starIndex: integer) => string
    newPage?: boolean
}> = (props) => {
    const [expand, setExpand] = createStore({
        detail: false,
        planets: {} as Record<number, boolean>,
    })

    const xStarPostions = createMemo(
        () =>
            props.galaxy?.stars
                .filter(
                    (star) =>
                        star.type === StarType.BlackHole ||
                        star.type === StarType.NeutronStar,
                )
                .map((star) => star.position),
    )

    return (
        <div class={styles.view}>
            <div class={styles.main}>
                <div class={styles.column}>
                    <div class={styles.card}>
                        <Expand
                            expand={expand.detail}
                            toggle={() => setExpand("detail", (x) => !x)}
                        />
                        <div class={styles.title}>
                            <span>{props.star.name}</span>
                            <span class={styles.index}>
                                #{props.star.index + 1}
                            </span>
                        </div>
                        <StarDetail
                            star={props.star}
                            expand={expand.detail}
                            positions={xStarPostions()}
                        />
                    </div>
                    <div class={styles.card}>
                        <div class={styles.title}>
                            <span>Resources</span>
                        </div>
                        <StarVeins star={props.star} />
                    </div>
                </div>
                <Show when={!!props.galaxy}>
                    <div class={styles.column}>
                        <div class={styles.card}>
                            <div class={styles.title}>
                                <span>Nearby Stars</span>
                            </div>
                            <For
                                each={nearbyStars(
                                    props.star,
                                    props.galaxy!.stars,
                                )}
                            >
                                {({ star, distance }) => (
                                    <NearbyStar
                                        seed={props.galaxy!.seed}
                                        star={star}
                                        distance={distance}
                                        url={props.buildUrl(star.index)}
                                        newPage={props.newPage}
                                    />
                                )}
                            </For>
                        </div>
                    </div>
                </Show>
            </div>
            <div class={styles.column}>
                <div class={clsx(styles.card, styles.planets)}>
                    <div class={styles.title}>
                        <span>Planets</span>
                    </div>
                    <For each={props.star.planets}>
                        {(planet) => (
                            <PlanetView star={props.star} planet={planet} />
                        )}
                    </For>
                </div>
            </div>
        </div>
    )
}

export default StarView
