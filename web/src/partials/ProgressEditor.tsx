import { Component, createMemo, Show } from "solid-js"
import Input from "../components/Input"
import styles from "~styles"
import StarCountSelector from "./StarCountSelector"
import Tooltip from "../components/Tooltip"
import Toggle from "../components/Toggle"
import ExeUrl from "../../../target/release/dsp_seed.exe?url"
import Button from "../components/Button"
import ResourceMultiplierSelector from "./ResourceMultiplierSelector"
import NumberInput from "../components/NumberInput"
import HiveInitialColonizeSelector from "./HiveInitialColonizeSelector"
import HiveMaxDensitySelector from "./HiveMaxDensitySelector"
import { Trans, useLingui } from "#lingui"
import type { SetStoreFunction } from "solid-js/store"
import { IoTrash } from "solid-icons/io"

function extractSeeds(contents: string[]): FindRange | null {
    const raw = new Set<integer>()
    const regex = /^\d{1,8}(?!\d)/gm
    for (const content of contents) {
        const extracted = content.match(regex)
        if (extracted) {
            for (const num of extracted) {
                raw.add(Number(num))
            }
        }
    }
    if (raw.size === 0) {
        return null
    }
    const output = [...raw]
    output.sort((a, b) => a - b)
    return new Int32Array<ArrayBuffer>(output as any)
}

const MAX_SIZE = 10 * 1024 * 1024

const SeedImport: Component<{
    value: FindRange
    onChange: (value: FindRange) => void
    disabled: boolean
}> = (props) => {
    const { t } = useLingui()

    const onChange = (seeds: FindRange | null) => {
        if (seeds) {
            props.onChange(seeds)
        }
    }

    const chooseFile = () => {
        const input = document.createElement("input")
        input.type = "file"
        input.accept = ".txt, .csv, .tsv"
        input.multiple = true
        input.onchange = () => {
            if (!input.files) return
            const files = Array.from(input.files)
            const size = files.reduce((acc, file) => acc + file.size, 0)
            if (size > MAX_SIZE) return
            Promise.all(files.map((file) => file.text()))
                .then(extractSeeds)
                .then(onChange)
        }
        input.click()
    }

    return (
        <Show
            when={props.value instanceof Int32Array}
            fallback={<Button onClick={chooseFile}>{t`Choose file`}</Button>}
        >
            <div class={styles.seedImport}>
                {t`Imported ${props.value.length} seeds`}
                <Show when={!props.disabled}>
                    <span
                        class={styles.delete}
                        onClick={() => onChange([0, 1e8])}
                    >
                        <IoTrash />
                    </span>
                </Show>
            </div>
        </Show>
    )
}

const ProgressEditor: Component<{
    progress: ProfileProgressInfo
    onProgressChange: SetStoreFunction<ProfileProgressInfo>
    name: string
    onNameChange: (v: string) => void
    nativeMode: boolean
    onNativeModeChange: (v: boolean) => void
    isLoaded: boolean
    searching: boolean
}> = (props) => {
    const hasProgress = () => props.progress.nextBatchId > 0
    const isDisabled = () => props.searching || hasProgress()

    const { t } = useLingui()
    const isUsingImportSeed = createMemo(
        () => props.progress.range instanceof Int32Array,
    )
    const seedStart = createMemo(() =>
        isUsingImportSeed() ? 0 : props.progress.range[0],
    )
    const seedEnd = createMemo(() =>
        isUsingImportSeed() ? 1e8 : props.progress.range[1],
    )

    return (
        <div class={styles.fields}>
            <div class={styles.field}>
                <div class={styles.label}>
                    {props.isLoaded ? t`Profile Name` : t`New Profile Name`}
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
            <div class={styles.field}>
                <div class={styles.label}>{t`Seed range`}</div>
                <div class={styles.input}>
                    <Trans>
                        <NumberInput
                            class={styles.inputSeed}
                            value={seedStart()}
                            onChange={(value) =>
                                props.onProgressChange("range", [
                                    value,
                                    seedEnd(),
                                ])
                            }
                            emptyValue={-1}
                            maxLength={8}
                            error={seedStart() < 0 || seedStart() >= seedEnd()}
                            disabled={isDisabled() || isUsingImportSeed()}
                        />{" "}
                        to{" "}
                        <NumberInput
                            class={styles.inputSeed}
                            value={seedEnd() - 1}
                            onChange={(value) =>
                                props.onProgressChange("range", [
                                    seedStart(),
                                    value + 1,
                                ])
                            }
                            emptyValue={-1}
                            maxLength={8}
                            error={seedEnd() > 1e8 || seedStart() >= seedEnd()}
                            disabled={isDisabled() || isUsingImportSeed()}
                        />
                    </Trans>
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>{t`Number of stars`}</div>
                <div class={styles.input}>
                    <StarCountSelector
                        class={styles.inputStandard}
                        value={props.progress.params.starCount}
                        onChange={(value) =>
                            props.onProgressChange("params", "starCount", value)
                        }
                        disabled={isDisabled()}
                    />
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>
                    <Tooltip
                        text={t`Provide a seed list to limit the search to these seeds only. Must be .txt / .csv / .tsv. One seed per line. Maximum 10MB.`}
                    >
                        {t`Import seeds`}
                    </Tooltip>
                </div>
                <div class={styles.input}>
                    <SeedImport
                        value={props.progress.range}
                        onChange={(value) =>
                            props.onProgressChange("range", value)
                        }
                        disabled={isDisabled()}
                    />
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>{t`Resource multiplier`}</div>
                <div class={styles.input}>
                    <ResourceMultiplierSelector
                        class={styles.inputStandard}
                        value={props.progress.params.resourceMultiplier}
                        onChange={(value) =>
                            props.onProgressChange(
                                "params",
                                "resourceMultiplier",
                                value,
                            )
                        }
                        disabled={isDisabled()}
                    />
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>
                    <Tooltip
                        text={t`To run the search in (faster) native mode, click the download button and run the program on your PC, then enable this option.`}
                    >
                        {t`Native Mode`}
                    </Tooltip>
                </div>
                <div class={styles.input}>
                    <Toggle
                        value={props.nativeMode}
                        onChange={props.onNativeModeChange}
                        disabled={props.searching}
                    />
                    <a href={ExeUrl} download="DSP-Seed-Finder.exe">
                        <Button kind="outline">{t`Download`}</Button>
                    </a>
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>{t`Dark Fog initial occupation`}</div>
                <div class={styles.input}>
                    <HiveInitialColonizeSelector
                        class={styles.inputStandard}
                        value={props.progress.params.hiveInitialColonize}
                        onChange={(value) =>
                            props.onProgressChange(
                                "params",
                                "hiveInitialColonize",
                                value,
                            )
                        }
                        disabled={isDisabled()}
                    />
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>
                    <Tooltip
                        text={t`The number of parallel processes to run the search.`}
                    >
                        {t`Concurrency`}
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
                <div class={styles.label}>{t`Dark Fog max density`}</div>
                <div class={styles.input}>
                    <HiveMaxDensitySelector
                        class={styles.inputStandard}
                        value={props.progress.params.hiveMaxDensity}
                        onChange={(value) =>
                            props.onProgressChange(
                                "params",
                                "hiveMaxDensity",
                                value,
                            )
                        }
                        disabled={isDisabled()}
                    />
                </div>
            </div>
            <div class={styles.field}>
                <div class={styles.label}>
                    <Tooltip
                        text={t`Running autosave too frequently may decrease search performance.`}
                    >
                        {t`Autosave interval`}
                    </Tooltip>
                </div>
                <div class={styles.input}>
                    <Trans>
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
                    </Trans>
                </div>
            </div>
        </div>
    )
}

export default ProgressEditor
