import { useLingui } from "#lingui"
import { Component, createSignal } from "solid-js"
import styles from "~styles"
import Toggle from "../components/Toggle"
import Tooltip from "../components/Tooltip"
import HiveInitialColonizeSelector from "../partials/HiveInitialColonizeSelector"
import HiveMaxDensitySelector from "../partials/HiveMaxDensitySelector"
import ResourceMultiplierSelector from "../partials/ResourceMultiplierSelector"
import StarCountSelector from "../partials/StarCountSelector"
import { createStore, unwrap } from "solid-js/store"
import { getDefaultParams } from "../util"
import ExeUrl from "../../../target/release/dsp_seed.exe?url"
import Button from "../components/Button"
import NumberInput from "../components/NumberInput"
import Input from "../components/Input"
import { startGeneratingDatabase } from "../worldgen"

const GenerateDatabase: Component = () => {
    const [params, setParams] = createStore(getDefaultParams())
    const [name, setName] = createSignal("dsp.sqlite3")
    const [start, setStart] = createSignal(0)
    const [end, setEnd] = createSignal(1e8)
    const [concurrency, setConcurrency] = createSignal(
        navigator.hardwareConcurrency,
    )

    function handleSubmit(ev: Event) {
        ev.preventDefault()
        startGeneratingDatabase({
            name: name(),
            range: [start(), end()],
            params: unwrap(params),
            concurrency: concurrency(),
            onProgress(seed) {
                console.log(seed)
            },
            onInterrupt() {
                console.log("interrupt")
            },
            onComplete() {
                console.log("complete")
            },
            onError(err) {
                console.error(err)
            },
        })
    }

    const { t } = useLingui()

    return (
        <form class={styles.search} onSubmit={handleSubmit}>
            <div class={styles.searchTitle}>
                <Tooltip
                    text={t`You must download the native program to generate seed database.`}
                >
                    {t`Native Mode`}
                </Tooltip>
                :
            </div>
            <a href={ExeUrl} download="DSP-Seed-Finder.exe">
                <Button kind="outline">{t`Download`}</Button>
            </a>
            <div class={styles.searchTitle}>{t`Database name`}:</div>
            <Input
                class={styles.searchInput}
                value={name()}
                onChange={setName}
                error={!name()}
            />
            <div class={styles.searchTitle}>{t`Seed range`}:</div>
            <div>
                <NumberInput
                    class={styles.inputSeed}
                    value={start()}
                    onChange={setStart}
                    emptyValue={-1}
                    maxLength={8}
                    error={start() < 0 || start() >= end()}
                />{" "}
                to{" "}
                <NumberInput
                    class={styles.inputSeed}
                    value={end() - 1}
                    onChange={(value) => setEnd(value + 1)}
                    emptyValue={-1}
                    maxLength={8}
                    error={end() > 1e8 || start() >= end()}
                />
            </div>
            <div class={styles.searchTitle}>{t`Concurrency`}:</div>
            <NumberInput
                class={styles.searchInput}
                value={concurrency()}
                onChange={setConcurrency}
                emptyValue={-1}
                maxLength={2}
                error={!Number.isInteger(concurrency()) || concurrency() < 1}
            />
            <div class={styles.searchTitle}>{t`Number of stars`}:</div>
            <StarCountSelector
                class={styles.searchInput}
                value={params.starCount}
                onChange={(v) => setParams("starCount", v)}
            />
            <div class={styles.searchTitle}>{t`Resource multiplier`}:</div>
            <ResourceMultiplierSelector
                class={styles.searchInput}
                value={params.resourceMultiplier}
                onChange={(v) => setParams("resourceMultiplier", v)}
            />
            <div class={styles.searchTitle}>
                {t`Dark Fog initial occupation`}:
            </div>
            <HiveInitialColonizeSelector
                class={styles.searchInput}
                value={params.hiveInitialColonize}
                onChange={(v) => setParams("hiveInitialColonize", v)}
            />
            <div class={styles.searchTitle}>{t`Dark Fog max density`}:</div>
            <HiveMaxDensitySelector
                class={styles.searchInput}
                value={params.hiveMaxDensity}
                onChange={(v) => setParams("hiveMaxDensity", v)}
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
                value={!params.useActualVeins}
                onChange={(v) => setParams("useActualVeins", !v)}
            />
            <Button type="submit">{t`Generate`}</Button>
        </form>
    )
}

export default GenerateDatabase
