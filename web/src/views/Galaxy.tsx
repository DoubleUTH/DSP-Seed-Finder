import styles from "./Galaxy.module.css"
import { A, useNavigate, useParams } from "@solidjs/router"
import { Component, For, Show, createEffect, createSignal } from "solid-js"
import NumberInput from "../components/NumberInput"
import Button from "../components/Button"
import { useWorldGen } from "../worldgen"
import { useStore } from "../store"
import clsx from "clsx"
import StarView from "../partials/StarView"

function randomSeed() {
    return Math.floor(Math.random() * 1e8)
}

const Search: Component = () => {
    const [value, setValue] = createSignal<number>(-1)
    const navigate = useNavigate()

    function isValueValid() {
        const v = value()
        return Number.isInteger(v) && v >= 0 && v < 1e8
    }

    function handleSubmit(ev: Event) {
        ev.preventDefault()
        if (!isValueValid()) return
        navigate(`/galaxy/${value()}/0`)
    }

    return (
        <form class={styles.search} onSubmit={handleSubmit}>
            <div class={styles.searchTitle}>Seed:</div>
            <NumberInput
                class={styles.searchInput}
                value={value()}
                onChange={setValue}
                min={0}
                max={99999999}
                step={1}
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
        </form>
    )
}

const View: Component<{ seed: number; index: number }> = (props) => {
    const [store, setStore] = useStore()
    const worldgen = useWorldGen()

    function requiresLoad() {
        const g = store.galaxys[props.seed]
        if (!g) return true
        if (g.loading) return false
        return (
            g.resourceMultiplier !== store.settings.resourceMultiplier ||
            g.starCount !== store.settings.starCount ||
            g.starCount !== g.stars.length
        )
    }

    function isAvailable() {
        const g = store.galaxys[props.seed]
        return !!g && !g.loading && !requiresLoad()
    }

    createEffect(() => {
        if (requiresLoad()) {
            const config = {
                seed: props.seed,
                starCount: store.settings.starCount,
                resourceMultiplier: store.settings.resourceMultiplier,
            }
            if (config.starCount < 32 || config.starCount > 64) return
            setStore("galaxys", config.seed, (v) =>
                v ? { loading: true } : { ...config, loading: true, stars: [] },
            )
            worldgen()
                .generate(config)
                .then((g): void => {
                    setStore("galaxys", config.seed, {
                        ...config,
                        loading: false,
                        stars: g.stars,
                    })
                })
        }
    })

    function stars() {
        return store.galaxys[props.seed]!.stars
    }

    return (
        <Show
            when={isAvailable()}
            fallback={<div class={styles.loading}>Loading...</div>}
        >
            <div class={styles.view}>
                <div class={styles.left}>
                    <For each={stars()}>
                        {(star) => (
                            <A
                                href={`/galaxy/${props.seed}/${star.index}`}
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
                        star={stars()[props.index]!}
                        galaxy={store.galaxys[props.seed]!}
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
