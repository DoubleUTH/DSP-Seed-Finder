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
import { getWorldGen } from "../worldgen"
import { useStore } from "../store"
import clsx from "clsx"
import StarView from "../partials/StarView"
import {
    defaultResourceMultipler,
    defaultStarCount,
    getSearch,
    maxStarCount,
    minStarCount,
    resourceMultiplers,
} from "../util"
import StarCountSelector from "../partials/StarCountSelector"
import ResourceMultiplierSelector from "../partials/ResourceMultiplerSelector"

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
        navigate(
            `/galaxy/${value()}/0${getSearch({
                count: store.settings.view.starCount,
                multipler: store.settings.view.resourceMultipler,
            })}`,
        )
    }

    return (
        <form class={styles.search} onSubmit={handleSubmit}>
            <div class={styles.searchTitle}>Seed:</div>
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
                    Random
                </Button>
                <Button
                    class={styles.searchSubmit}
                    type="submit"
                    disabled={!isValueValid()}
                >
                    View
                </Button>
            </div>
            <div class={styles.searchTitle}>Star count:</div>
            <StarCountSelector
                class={styles.searchInput}
                value={store.settings.view.starCount}
                onChange={(v) => setStore("settings", "view", "starCount", v)}
            />
            <div class={styles.searchTitle}>Resource Multipler:</div>
            <ResourceMultiplierSelector
                class={styles.searchInput}
                value={store.settings.view.resourceMultipler}
                onChange={(v) =>
                    setStore("settings", "view", "resourceMultipler", v)
                }
            />
        </form>
    )
}

const View: Component<{ seed: number; index: number }> = (props) => {
    const [searchParams] = useSearchParams()

    const starCount = createMemo(() => {
        const { count } = searchParams
        if (count) {
            const m = parseFloat(count)
            if (Number.isInteger(m) && m >= minStarCount && m <= maxStarCount) {
                return m
            }
        }
        return defaultStarCount
    })

    const resourcMultipler = createMemo(() => {
        const { multipler } = searchParams
        if (multipler) {
            const m = parseFloat(multipler)
            if (resourceMultiplers.includes(m)) {
                return m
            }
        }
        return defaultResourceMultipler
    })

    const [galaxy] = createResource<Galaxy>(async () => {
        const config = {
            seed: props.seed,
            starCount: starCount(),
            resourceMultiplier: resourcMultipler(),
        }
        return getWorldGen(false).generate(config)
    })

    const search = createMemo(() =>
        getSearch({
            count: starCount(),
            multipler: resourcMultipler(),
        }),
    )

    function buildUrl(starIndex: integer) {
        return `/galaxy/${props.seed}/${starIndex}${search()}`
    }

    return (
        <Show
            when={!!galaxy()}
            fallback={<div class={styles.loading}>Loading...</div>}
        >
            <div class={styles.view}>
                <div class={styles.left}>
                    <For each={galaxy()!.stars}>
                        {(star) => (
                            <A
                                href={buildUrl(star.index)}
                                class={clsx(
                                    styles.star,
                                    star.index === props.index && styles.active,
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
                <div class={styles.right}>
                    <StarView
                        star={galaxy()!.stars[props.index]!}
                        galaxy={galaxy()!}
                        buildUrl={buildUrl}
                    />
                </div>
            </div>
        </Show>
    )
}

const Galaxy: Component = () => {
    const params = useParams()

    return (
        <Show when={!!params.seed} fallback={<Search />}>
            <View
                seed={Number(params.seed)}
                index={Number(params.index) || 0}
            />
        </Show>
    )
}

export default Galaxy
