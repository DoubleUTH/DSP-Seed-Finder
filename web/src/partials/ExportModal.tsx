import {
    Component,
    batch,
    createEffect,
    createSignal,
    onCleanup,
} from "solid-js"
import Modal from "../components/Modal"
import Button from "../components/Button"
import { getMultiProfileResultRange, getProfileResultRange } from "../profile"
import { getExporter } from "../exporter"
import { createStore } from "solid-js/store/types/server.js"
import { unwrap } from "solid-js/store"
import { TinyEmitter } from "tiny-emitter"
import styles from "./ExportModal.module.css"

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
    const getBlob = await getExporter(false)({
        format,
        concurrency,
        starCount,
        resourceMultiplier,
        exportAllStars: mode === "galaxy" || exportAllStars,
        results: results,
        onProgress: (current) => {
            emitter.emit("progress", current)
        },
    })
    emitter.emit("end")
    const blob = await getBlob()
    return blob
}

enum Status {
    Idle,
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
    id: string
}> = (props) => {
    const [progress, setProgress] = createSignal(0)
    const [total, setTotal] = createSignal(0)
    const [status, setStatus] = createSignal<Status>(Status.Idle)
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
            default:
                return ""
        }
    }

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
            execute(emitter, props.mode, props.id, props.options).then(
                (blob) => {
                    setStatus(Status.Done)
                    const url = URL.createObjectURL(blob)
                    setUrl((prevUrl) => {
                        if (prevUrl) {
                            URL.revokeObjectURL(prevUrl)
                        }
                        return url
                    })
                },
            )
        } else {
            batch(() => {
                setStatus(Status.Idle)
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
        setUrl((prevUrl) => {
            if (prevUrl) {
                URL.revokeObjectURL(prevUrl)
            }
            return ""
        })
    })

    return (
        <Modal visible={props.visible}>
            <Button onClick={props.onClose}>Close</Button>
        </Modal>
    )
}

const ExportModal: Component<{
    visible: boolean
    onClose: () => void
    mode: Mode
    id: string
    starCount: integer
    resourceMultiplier: float
}> = (props) => {
    const [options, setOptions] = createStore<Options>({
        start: 0,
        end: 99999999,
        // eslint-disable-next-line solid/reactivity
        starCount: props.starCount,
        // eslint-disable-next-line solid/reactivity
        resourceMultiplier: props.resourceMultiplier,
        format: "xlsx",
        concurrency: 8,
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

    return (
        <Modal visible={props.visible} onClose={props.onClose} backdropDismiss>
            <ProgressModal
                visible={progressModal()}
                onClose={() => setProgressModal(false)}
                mode={props.mode}
                id={props.id}
                options={unwrap(options)}
            />
        </Modal>
    )
}

export default ExportModal
