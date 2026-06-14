import {
    Component,
    Show,
    batch,
    createEffect,
    createMemo,
    createSignal,
    onCleanup,
} from "solid-js"
import Modal from "../components/Modal"
import Button from "../components/Button"
import { getMultiProfileResultRange, getProfileResultRange } from "../profile"
import { getExporter } from "../exporter"
import { createStore, unwrap } from "solid-js/store"
import { TinyEmitter } from "tiny-emitter"
import styles from "./ExportModal.module.css"
import StarCountSelector from "./StarCountSelector"
import ResourceMultiplierSelector from "./ResourceMultiplierSelector"
import NumberInput from "../components/NumberInput"
import Tooltip from "../components/Tooltip"
import Toggle from "../components/Toggle"
import Select from "../components/Select"
import { getDefaultParams } from "../util"
import HiveInitialColonizeSelector from "./HiveInitialColonizeSelector"
import HiveMaxDensitySelector from "./HiveMaxDensitySelector"
import { useLingui } from "#lingui"
import { useStore } from "../store"

type Mode = "star" | "galaxy" | "single"

interface Options extends Pick<
    ExportOptions,
    "format" | "concurrency" | "exportAllStars" | "params" | "language"
> {
    start: number
    end: number
}

function formatName(name: string, format: ExportOptions["format"]) {
    return (
        name.replace(/\\\/:*?"<>|/g, "") +
        "." +
        (format === "csv" ? "zip" : format)
    )
}

async function getStarResults(
    id: string,
    start: number,
    end: number,
): Promise<FindResult[]> {
    const results = await getProfileResultRange(id, start, end)
    const output: FindResult[] = []
    let last: FindResult | undefined
    for (const result of results) {
        if (last?.seed === result.seed) {
            last.indexes.push(result.index)
        } else {
            last = {
                seed: result.seed,
                indexes: [result.index],
            }
            output.push(last)
        }
    }
    return output
}

async function getGalaxyResults(
    id: string,
    start: number,
    end: number,
): Promise<FindResult[]> {
    const results = await getMultiProfileResultRange(id, start, end)
    return results as FindResult[]
}

async function execute(
    emitter: TinyEmitter,
    mode: Mode,
    id: string,
    {
        start,
        end,
        format,
        concurrency,
        exportAllStars,
        params,
        language,
    }: Options,
) {
    const fn =
        mode === "star"
            ? getStarResults
            : mode === "galaxy"
              ? getGalaxyResults
              : () => [{ seed: start, indexes: [] }]
    const results = await fn(id, start, end)
    if (format === "txt") {
        const content = results.map(({ seed }) => seed).join("\n")
        return new Blob([content], { type: "text/plain" })
    }
    emitter.emit("start", results.length)
    let stopped = false
    emitter.once("stop", () => {
        stopped = true
    })
    const blob = await getExporter(false)({
        format,
        concurrency,
        params,
        exportAllStars: mode !== "star" || exportAllStars,
        results,
        language,
        onProgress: (current) => {
            emitter.emit("progress", current)
            return stopped
        },
        onGenerate: () => emitter.emit("end"),
    })
    return blob
}

enum Status {
    Starting,
    Progressing,
    Generating,
    Done,
}

const ProgressModal: Component<{
    visible: boolean
    onClose: () => void
    mode: Mode
    options: Options
    name: string
    id: string
}> = (props) => {
    const [progress, setProgress] = createSignal(0)
    const [total, setTotal] = createSignal(0)
    const [status, setStatus] = createSignal<Status>(Status.Done)
    const [url, setUrl] = createSignal("")
    const { t } = useLingui()

    const progressText = () => {
        switch (status()) {
            case Status.Starting:
                return t`Retriving data`
            case Status.Progressing:
                return t`Exporting ${progress()} / ${total()}`
            case Status.Generating:
                return t`Generating file`
            case Status.Done:
                return t`Done`
        }
    }

    let stop = () => {}

    createEffect(() => {
        if (props.visible) {
            const emitter = new TinyEmitter()
            setStatus(Status.Starting)
            emitter.once("start", (count: integer) => {
                batch(() => {
                    setProgress(0)
                    setTotal(count)
                    setStatus(Status.Progressing)
                })
            })
            emitter.on("progress", (current: integer) => {
                setProgress(current)
            })
            emitter.once("end", () => {
                setStatus(Status.Generating)
            })
            stop = () => emitter.emit("stop")
            execute(emitter, props.mode, props.id, props.options).then(
                (blob) => {
                    if (blob) {
                        setStatus(Status.Done)
                        const url = URL.createObjectURL(blob)
                        setUrl((prevUrl) => {
                            if (prevUrl) {
                                URL.revokeObjectURL(prevUrl)
                            }
                            return url
                        })
                    }
                },
            )
        } else {
            stop()
            stop = () => {}
            batch(() => {
                setUrl((prevUrl) => {
                    if (prevUrl) {
                        URL.revokeObjectURL(prevUrl)
                    }
                    return ""
                })
            })
        }
    })

    onCleanup(() => {
        stop()
        stop = () => {}
        setUrl((prevUrl) => {
            if (prevUrl) {
                URL.revokeObjectURL(prevUrl)
            }
            return ""
        })
    })

    return (
        <Modal visible={props.visible}>
            <div class={styles.progressText}>{progressText()}</div>
            <Show when={url()}>
                <a class={styles.download} download={props.name} href={url()}>
                    <Button class={styles.button}>{t`Download`}</Button>
                </a>
            </Show>
            <Button
                class={styles.button}
                kind="outline"
                onClick={props.onClose}
            >
                {status() === Status.Done ? t`Close` : t`Stop`}
            </Button>
        </Modal>
    )
}

const ExportModal: Component<{
    visible: boolean
    onClose: () => void
    mode: Mode
    id: string
    singleSeed?: integer
    name: string
    params: GameParameters
}> = (props) => {
    const [store] = useStore()
    const [options, setOptions] = createStore<Options>({
        start: 0,
        end: 99999999,
        params: getDefaultParams(),
        format: "xlsx",
        concurrency: navigator.hardwareConcurrency,
        exportAllStars: false,
        language: store.settings.language,
    })
    const { t } = useLingui()

    const [progressModal, setProgressModal] = createSignal(false)

    createEffect(() => {
        if (props.visible) {
            setOptions({
                params: { ...props.params },
                language: store.settings.language,
                start: props.singleSeed ?? 0,
            })
        } else {
            setProgressModal(false)
        }
    })

    const filename = createMemo(() => formatName(props.name, options.format))

    return (
        <Modal visible={props.visible} onClose={props.onClose} backdropDismiss>
            <div class={styles.title}>{t`Export`}</div>
            <Show when={props.mode !== "single" && options.format !== "txt"}>
                <div class={styles.warn}>
                    {t`Warning: Exporting too many seeds may cause out of memory error.`}
                </div>
            </Show>
            <div class={styles.fields}>
                <div class={styles.label}>{t`Format`}</div>
                <div class={styles.input}>
                    <Select
                        class={styles.inputStandard}
                        value={options.format}
                        onChange={(value) => setOptions("format", value)}
                        options={
                            props.mode === "single"
                                ? (["xlsx", "csv"] as const)
                                : (["xlsx", "csv", "txt"] as const)
                        }
                        getLabel={(value) =>
                            value === "txt" ? t`Seed only` : value
                        }
                    />
                </div>
                <Show when={options.format !== "txt"}>
                    <div class={styles.label}>{t`Number of stars`}</div>
                    <div class={styles.input}>
                        <StarCountSelector
                            class={styles.inputStandard}
                            value={options.params.starCount}
                            onChange={(value) =>
                                setOptions("params", "starCount", value)
                            }
                        />
                    </div>
                    <div class={styles.label}>{t`Resource multiplier`}</div>
                    <div class={styles.input}>
                        <ResourceMultiplierSelector
                            class={styles.inputStandard}
                            value={options.params.resourceMultiplier}
                            onChange={(value) =>
                                setOptions(
                                    "params",
                                    "resourceMultiplier",
                                    value,
                                )
                            }
                        />
                    </div>
                    <div
                        class={styles.label}
                    >{t`Dark Fog initial occupation`}</div>
                    <div class={styles.input}>
                        <HiveInitialColonizeSelector
                            class={styles.inputStandard}
                            value={options.params.hiveInitialColonize}
                            onChange={(value) =>
                                setOptions(
                                    "params",
                                    "hiveInitialColonize",
                                    value,
                                )
                            }
                        />
                    </div>
                    <div class={styles.label}>{t`Dark Fog max density`}</div>
                    <div class={styles.input}>
                        <HiveMaxDensitySelector
                            class={styles.inputStandard}
                            value={options.params.hiveMaxDensity}
                            onChange={(value) =>
                                setOptions("params", "hiveMaxDensity", value)
                            }
                        />
                    </div>
                    <div class={styles.label}>
                        <Tooltip
                            text={t`It is much faster to estimate the amount of veins over generating the excat numbers.`}
                        >
                            {t`Use estimated veins`}
                        </Tooltip>
                        :
                    </div>
                    <div class={styles.input}>
                        <Toggle
                            value={!options.params.useActualVeins}
                            onChange={(value) =>
                                setOptions("params", "useActualVeins", !value)
                            }
                        />
                    </div>
                </Show>
                <Show when={props.mode !== "single"}>
                    <div class={styles.label}>{t`Seed range`}</div>
                    <div class={styles.input}>
                        <NumberInput
                            class={styles.inputSeed}
                            value={options.start}
                            onChange={(value) => setOptions("start", value)}
                            emptyValue={-1}
                            maxLength={8}
                            error={
                                options.start < 0 ||
                                options.start >= options.end
                            }
                        />{" "}
                        to{" "}
                        <NumberInput
                            class={styles.inputSeed}
                            value={options.end}
                            onChange={(value) => setOptions("end", value)}
                            emptyValue={-1}
                            maxLength={8}
                            error={
                                options.end > 1e8 ||
                                options.start >= options.end
                            }
                        />
                    </div>
                </Show>
                <Show when={props.mode === "star" && options.format !== "txt"}>
                    <div class={styles.label}>
                        <Tooltip
                            text={t`Export all stars instead of only the matching ones`}
                        >
                            {t`Export all`}
                        </Tooltip>
                    </div>
                    <div class={styles.input}>
                        <Toggle
                            value={options.exportAllStars}
                            onChange={(value) =>
                                setOptions("exportAllStars", value)
                            }
                        />
                    </div>
                </Show>
                <Show
                    when={props.mode !== "single" && options.format !== "txt"}
                >
                    <div class={styles.label}>{t`Concurrency`}</div>
                    <div class={styles.input}>
                        <NumberInput
                            class={styles.inputStandard}
                            value={options.concurrency}
                            onChange={(value) =>
                                setOptions("concurrency", value)
                            }
                            emptyValue={-1}
                            maxLength={2}
                            error={
                                !Number.isInteger(options.concurrency) ||
                                options.concurrency < 1
                            }
                        />
                    </div>
                </Show>
            </div>
            <Button
                class={styles.button}
                onClick={() => setProgressModal(true)}
            >
                {t`Export`}
            </Button>
            <ProgressModal
                visible={progressModal()}
                onClose={() => setProgressModal(false)}
                mode={props.mode}
                id={props.id}
                name={filename()}
                options={unwrap(options)}
            />
        </Modal>
    )
}

export default ExportModal
