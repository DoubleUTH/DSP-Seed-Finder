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
import ResourceMultiplierSelector from "./ResourceMultiplerSelector"
import NumberInput from "../components/NumberInput"
import Tooltip from "../components/Tooltip"
import Toggle from "../components/Toggle"
import Select from "../components/Select"

type Mode = "star" | "galaxy"

interface Options
    extends Pick<
        ExportOptions,
        | "format"
        | "concurrency"
        | "exportAllStars"
        | "starCount"
        | "resourceMultiplier"
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
        starCount,
        resourceMultiplier,
    }: Options,
) {
    const fn = mode === "star" ? getStarResults : getGalaxyResults
    const results = await fn(id, start, end)
    emitter.emit("start", results.length)
    let stopped = false
    emitter.once("stop", () => {
        stopped = true
    })
    const blob = await getExporter(false)({
        format,
        concurrency,
        starCount,
        resourceMultiplier,
        exportAllStars: mode === "galaxy" || exportAllStars,
        results: results,
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

    const progressText = () => {
        switch (status()) {
            case Status.Starting:
                return "Retriving Data"
            case Status.Progressing:
                return `Exporting ${progress()} / ${total()}`
            case Status.Generating:
                return "Generating file"
            case Status.Done:
                return "Done"
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
                    <Button class={styles.button}>Download</Button>
                </a>
            </Show>
            <Button
                class={styles.button}
                kind="outline"
                onClick={props.onClose}
            >
                {status() === Status.Done ? "Close" : "Stop"}
            </Button>
        </Modal>
    )
}

const ExportModal: Component<{
    visible: boolean
    onClose: () => void
    mode: Mode
    id: string
    name: string
    starCount: integer
    resourceMultiplier: float
}> = (props) => {
    const [options, setOptions] = createStore<Options>({
        start: 0,
        end: 99999999,
        starCount: 0,
        resourceMultiplier: 0,
        format: "csv",
        concurrency: navigator.hardwareConcurrency,
        exportAllStars: false,
    })

    const [progressModal, setProgressModal] = createSignal(false)

    createEffect(() => {
        if (props.visible) {
            setOptions({
                starCount: props.starCount,
                resourceMultiplier: props.resourceMultiplier,
            })
        } else {
            setProgressModal(false)
        }
    })

    const filename = createMemo(() => formatName(props.name, options.format))

    return (
        <Modal visible={props.visible} onClose={props.onClose} backdropDismiss>
            <div class={styles.title}>Export</div>
            <div class={styles.warn}>
                Warning: Exporting too many seeds may cause out of memory error.
            </div>
            <div class={styles.fields}>
                <div class={styles.label}>Format</div>
                <div class={styles.input}>
                    <Select
                        class={styles.inputStandard}
                        value={options.format}
                        onChange={(value) => setOptions("format", value)}
                        options={["csv", "xlsx"] as const}
                        getLabel={(value) => value}
                    />
                </div>
                <div class={styles.label}>Star Count</div>
                <div class={styles.input}>
                    <StarCountSelector
                        class={styles.inputStandard}
                        value={options.starCount}
                        onChange={(value) => setOptions("starCount", value)}
                    />
                </div>
                <div class={styles.label}>Resource Multiplier</div>
                <div class={styles.input}>
                    <ResourceMultiplierSelector
                        class={styles.inputStandard}
                        value={options.resourceMultiplier}
                        onChange={(value) =>
                            setOptions("resourceMultiplier", value)
                        }
                    />
                </div>
                <div class={styles.label}>Seed Range</div>
                <div class={styles.input}>
                    <NumberInput
                        class={styles.inputSeed}
                        value={options.start}
                        onChange={(value) => setOptions("start", value)}
                        emptyValue={-1}
                        maxLength={8}
                        error={
                            options.start < 0 || options.start >= options.end
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
                            options.end > 1e8 || options.start >= options.end
                        }
                    />
                </div>
                <Show when={props.mode === "star"}>
                    <div class={styles.label}>
                        <Tooltip text="Export all stars instead of only the matching ones">
                            Export all
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
                <div class={styles.label}>Concurrency</div>
                <div class={styles.input}>
                    <NumberInput
                        class={styles.inputStandard}
                        value={options.concurrency}
                        onChange={(value) => setOptions("concurrency", value)}
                        emptyValue={-1}
                        maxLength={2}
                        error={
                            !Number.isInteger(options.concurrency) ||
                            options.concurrency < 1
                        }
                    />
                </div>
            </div>
            <Button
                class={styles.button}
                onClick={() => setProgressModal(true)}
            >
                Export
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
