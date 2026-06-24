import styles from "./Galaxy.module.css"
import { A, useNavigate, useParams, useSearchParams } from "@solidjs/router"
import {
    Component,
    For,
    Show,
    createMemo,
    createResource,
    createSignal,
} from "solid-js"
import NumberInput from "../components/NumberInput"
import Button from "../components/Button"
import { generateGalaxy } from "../worldgen"
import { useStore } from "../store"
import clsx from "clsx"
import StarView from "../partials/StarView"
import {
    defaultHiveInitialColonize,
    defaultHiveMaxDensity,
    defaultResourceMultiplier,
    defaultStarCount,
    defaultUseActualVeins,
    getSearch,
    hiveInitialColonizeValues,
    hiveMaxDensityValues,
    maxStarCount,
    minStarCount,
    resourceMultipliers,
} from "../util"
import StarCountSelector from "../partials/StarCountSelector"
import ResourceMultiplierSelector from "../partials/ResourceMultiplierSelector"
import HiveInitialColonizeSelector from "../partials/HiveInitialColonizeSelector"
import HiveMaxDensitySelector from "../partials/HiveMaxDensitySelector"
import { useLingui } from "#lingui"
import ExportModal from "../partials/ExportModal"
import Starmap from "../partials/Starmap"
import Tooltip from "../components/Tooltip"
import Toggle from "../components/Toggle"
import GalaxyOverview from "../partials/GalaxyOverview"

function randomSeed() {
    return Math.floor(Math.random() * 1e8)
}

const Search: Component = () => {
    const [store, setStore] = useStore()
    const [value, setValue] = createSignal<number>(-1)
    const navigate = useNavigate()

    function isValueValid() {
        const v = value()
        return Number.isInteger(v) && v >= 0 && v < 1e8
    }

    function handleSubmit(ev: Event) {
        ev.preventDefault()
        if (!isValueValid()) return
        navigate(`/galaxy/${value()}${getSearch(store.settings.view)}`)
    }

    const { t } = useLingui()

    return (
        <form class={styles.search} onSubmit={handleSubmit}>
            <div class={styles.searchTitle}>{t`Seed`}:</div>
            <div class={styles.searchRow}>
                <NumberInput
                    class={styles.searchInput}
                    value={value()}
                    onChange={setValue}
                    emptyValue={-1}
                />
                <Button
                    class={styles.searchRandom}
                    kind="outline"
                    onClick={() => setValue(randomSeed())}
                >
                    {t`Random`}
                </Button>
                <Button
                    class={styles.searchSubmit}
                    type="submit"
                    disabled={!isValueValid()}
                >
                    {t`View`}
                </Button>
            </div>
            <div class={styles.searchTitle}>{t`Number of stars`}:</div>
            <StarCountSelector
                class={styles.searchInput}
                value={store.settings.view.starCount}
                onChange={(v) => setStore("settings", "view", "starCount", v)}
            />
            <div class={styles.searchTitle}>{t`Resource multiplier`}:</div>
            <ResourceMultiplierSelector
                class={styles.searchInput}
                value={store.settings.view.resourceMultiplier}
                onChange={(v) =>
                    setStore("settings", "view", "resourceMultiplier", v)
                }
            />
            <div class={styles.searchTitle}>
                {t`Dark Fog initial occupation`}:
            </div>
            <HiveInitialColonizeSelector
                class={styles.searchInput}
                value={store.settings.view.hiveInitialColonize}
                onChange={(v) =>
                    setStore("settings", "view", "hiveInitialColonize", v)
                }
            />
            <div class={styles.searchTitle}>{t`Dark Fog max density`}:</div>
            <HiveMaxDensitySelector
                class={styles.searchInput}
                value={store.settings.view.hiveMaxDensity}
                onChange={(v) =>
                    setStore("settings", "view", "hiveMaxDensity", v)
                }
            />
            <div class={styles.searchTitle}>
                <Tooltip
                    text={t`It is much faster to estimate the amount of veins over generating the excat numbers.`}
                >
                    {t`Use estimated veins`}
                </Tooltip>
                :
            </div>
            <Toggle
                value={!store.settings.view.useActualVeins}
                onChange={(v) =>
                    setStore("settings", "view", "useActualVeins", !v)
                }
            />
        </form>
    )
}

const View: Component<{ seed: number; index?: number }> = (props) => {
    const [searchParams] = useSearchParams()
    const [exportModal, setExportModal] = createSignal(false)

    const starCount = createMemo(() => {
        const { count } = searchParams
        if (count) {
            const m = parseFloat(count as string)
            if (Number.isInteger(m) && m >= minStarCount && m <= maxStarCount) {
                return m
            }
        }
        return defaultStarCount
    })

    const resourceMultiplier = createMemo(() => {
        const { multiplier } = searchParams
        if (multiplier) {
            const m = parseFloat(multiplier as string)
            if (resourceMultipliers.includes(m)) {
                return m
            }
        }
        return defaultResourceMultiplier
    })

    const hiveInitialColonize = createMemo(() => {
        const { hiveInitialColonize } = searchParams
        if (hiveInitialColonize) {
            const m = parseFloat(hiveInitialColonize as string)
            if (hiveInitialColonizeValues.includes(m)) {
                return m
            }
        }
        return defaultHiveInitialColonize
    })

    const hiveMaxDensity = createMemo(() => {
        const { hiveMaxDensity } = searchParams
        if (hiveMaxDensity) {
            const m = parseFloat(hiveMaxDensity as string)
            if (hiveMaxDensityValues.includes(m)) {
                return m
            }
        }
        return defaultHiveMaxDensity
    })

    const useActualVeins = createMemo(() => {
        const { useActualVeins } = searchParams
        if (useActualVeins) {
            if (defaultUseActualVeins) {
                return useActualVeins !== "0" && useActualVeins !== "false"
            } else {
                return useActualVeins === "1" || useActualVeins === "true"
            }
        }
        return defaultUseActualVeins
    })

    const [galaxy] = createResource<Galaxy>(async () => {
        const config: GameParameters = {
            starCount: starCount(),
            resourceMultiplier: resourceMultiplier(),
            hiveInitialColonize: hiveInitialColonize(),
            hiveMaxDensity: hiveMaxDensity(),
            useActualVeins: useActualVeins(),
        }
        const galaxy = await generateGalaxy(false, props.seed, config)
        console.log(galaxy)
        return galaxy
    })

    const search = createMemo(() =>
        getSearch({
            starCount: starCount(),
            resourceMultiplier: resourceMultiplier(),
            hiveInitialColonize: hiveInitialColonize(),
            hiveMaxDensity: hiveMaxDensity(),
            useActualVeins: useActualVeins(),
        }),
    )

    function buildUrl(starIndex: integer) {
        return `/galaxy/${props.seed}/${starIndex}${search()}`
    }

    const { t } = useLingui()

    return (
        <Show
            when={!!galaxy()}
            fallback={<div class={styles.loading}>{t`Loading...`}</div>}
        >
            <div class={styles.view}>
                <div class={styles.left}>
                    <div class={styles.leftButtons}>
                        <A href={`/galaxy/${props.seed}${search()}`}>
                            <Button class={styles.export}>{t`Starmap`}</Button>
                        </A>
                        <Button
                            class={styles.export}
                            onClick={() => setExportModal(true)}
                        >{t`Export`}</Button>
                    </div>
                    <div class={styles.starList}>
                        <For each={galaxy()!.stars}>
                            {(star) => (
                                <A
                                    href={buildUrl(star.index)}
                                    class={clsx(
                                        styles.star,
                                        star.index === props.index &&
                                            styles.active,
                                    )}
                                >
                                    <span>{star.name}</span>
                                    <span class={styles.index}>
                                        #{star.index + 1}
                                    </span>
                                </A>
                            )}
                        </For>
                    </div>
                </div>
                <div class={styles.right}>
                    <Show
                        when={props.index !== undefined}
                        fallback={
                            <GalaxyOverview
                                galaxy={galaxy()!}
                                search={search()}
                            />
                        }
                    >
                        <StarView
                            star={galaxy()!.stars[props.index!]!}
                            galaxy={galaxy()!}
                            buildUrl={buildUrl}
                        />
                    </Show>
                </div>
            </div>
            <ExportModal
                visible={exportModal()}
                onClose={() => setExportModal(false)}
                mode="single"
                id=""
                name={String(props.seed)}
                singleSeed={props.seed}
                params={{
                    starCount: starCount(),
                    resourceMultiplier: resourceMultiplier(),
                    hiveInitialColonize: hiveInitialColonize(),
                    hiveMaxDensity: hiveMaxDensity(),
                    useActualVeins: useActualVeins(),
                }}
            />
        </Show>
    )
}

const Galaxy: Component = () => {
    const params = useParams()

    return (
        <Show when={!!params.seed} fallback={<Search />}>
            <View
                seed={Number(params.seed)}
                index={
                    params.index !== undefined
                        ? Number(params.index) || 0
                        : undefined
                }
            />
        </Show>
    )
}

export default Galaxy
