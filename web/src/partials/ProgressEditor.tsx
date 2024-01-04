import { JSX } from "solid-js"
import Input from "../components/Input"
import styles from "./ProgressEditor.module.css"
import StarCountSelector from "./StarCountSelector"
import Tooltip from "../components/Tooltip"
import Toggle from "../components/Toggle"
import ExeUrl from "../../../target/release/dsp_seed.exe?url"
import Button from "../components/Button"
import ResourceMultiplierSelector from "./ResourceMultiplerSelector"
import NumberInput from "../components/NumberInput"

function ProgressEditor<E extends ProfileProgressInfo>(props: {
    progress: E
    onProgressChange: <K extends keyof E>(key: K, v: E[K]) => void
    name: string
    onNameChange: (v: string) => void
    nativeMode: boolean
    onNativeModeChange: (v: boolean) => void
    isLoaded: boolean
    searching: boolean
}): JSX.Element {
    const hasProgress = () =>
        props.progress.start > -1 &&
        props.progress.current > props.progress.start
    const isDisabled = () => props.searching || hasProgress()

    return (
        <div class={styles.fields}>
            <div class={styles.field}>
                <div class={styles.label}>
                    {props.isLoaded ? "" : "New "}Profile Name
                </div>
                <div class={styles.input}>
                    <Input
                        value={props.name}
                        onChange={props.onNameChange}
                        error={props.name === ""}
                        disabled={props.searching}
                    />
                </div>
            </div>
            <div />
            <div class={styles.field}>
                <div class={styles.label}>Star count</div>
                <div class={styles.input}>
                    <StarCountSelector
                        class={styles.inputStandard}
                        value={props.progress.starCount}
                        onChange={(value) =>
                            props.onProgressChange("starCount", value)
                        }
                        disabled={isDisabled()}
                    />
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>
                    <Tooltip text="To run the search in (faster) native mode, click the download button and run the program on your PC, then enable this option.">
                        Native Mode
                    </Tooltip>
                </div>
                <div class={styles.input}>
                    <Toggle
                        value={props.nativeMode}
                        onChange={props.onNativeModeChange}
                        disabled={props.searching}
                    />
                    <a href={ExeUrl} download>
                        <Button kind="outline">Download</Button>
                    </a>
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>Resource Multipler</div>
                <div class={styles.input}>
                    <ResourceMultiplierSelector
                        class={styles.inputStandard}
                        value={props.progress.resourceMultiplier}
                        onChange={(value) =>
                            props.onProgressChange("resourceMultiplier", value)
                        }
                        disabled={isDisabled()}
                    />
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>
                    <Tooltip text="The number of parallel processes to run the search.">
                        Concurrency
                    </Tooltip>
                </div>
                <div class={styles.input}>
                    <NumberInput
                        class={styles.inputStandard}
                        value={props.progress.concurrency}
                        onChange={(value) =>
                            props.onProgressChange("concurrency", value)
                        }
                        emptyValue={-1}
                        maxLength={2}
                        error={
                            !Number.isInteger(props.progress.concurrency) ||
                            props.progress.concurrency < 1
                        }
                        disabled={props.searching}
                    />
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>Seed Range</div>
                <div class={styles.input}>
                    <NumberInput
                        class={styles.inputSeed}
                        value={props.progress.start}
                        onChange={(value) =>
                            props.onProgressChange("start", value)
                        }
                        emptyValue={-1}
                        maxLength={8}
                        error={
                            props.progress.start < 0 ||
                            props.progress.start >= props.progress.end
                        }
                        disabled={isDisabled()}
                    />{" "}
                    to{" "}
                    <NumberInput
                        class={styles.inputSeed}
                        value={props.progress.end - 1}
                        onChange={(value) =>
                            props.onProgressChange("end", value + 1)
                        }
                        emptyValue={-1}
                        maxLength={8}
                        error={
                            props.progress.end > 1e8 ||
                            props.progress.start >= props.progress.end
                        }
                        disabled={isDisabled()}
                    />
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>
                    <Tooltip text="Running autosave too frequently may decrease search performance.">
                        Autosave interval
                    </Tooltip>
                </div>
                <div class={styles.input}>
                    Every{" "}
                    <NumberInput
                        class={styles.inputSmall}
                        value={props.progress.autosave}
                        onChange={(value) =>
                            props.onProgressChange("autosave", value)
                        }
                        emptyValue={-1}
                        error={props.progress.autosave <= 0}
                        disabled={props.searching}
                    />{" "}
                    seconds
                </div>
            </div>
        </div>
    )
}

export default ProgressEditor
