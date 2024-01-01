import {
    Component,
    Match,
    Show,
    Switch,
    batch,
    createEffect,
    createMemo,
    createSignal,
} from "solid-js"
import Button from "../components/Button"
import { getWorldGen } from "../worldgen"
import { ConditionType, OceanType, RuleType, SpectrType } from "../enums"
import {
    deleteProfile,
    generateProfileId,
    getProfileProgress,
    listProfiles,
    setProfileInfo,
    setProfileProgress,
} from "../profile"
import { constructRule, maxStarCount, minStarCount } from "../util"
import RuleEditor from "../partials/RuleEditor"
import styles from "./Find.module.css"
import Input from "../components/Input"
import NumberInput from "../components/NumberInput"
import { createStore, unwrap } from "solid-js/store"
import StarCountSelector from "../partials/StarCountSelector"
import ResourceMultiplierSelector from "../partials/ResourceMultiplerSelector"
import Tooltip from "../components/Tooltip"
import ProfilesModal from "../partials/ProfilesModal"
import Modal from "../components/Modal"
import Toggle from "../components/Toggle"
import ProgressBar from "../components/ProgressBar"

const defaultProgress: ProfileProgress = {
    id: "",
    starCount: 64,
    resourceMultiplier: 1,
    concurrency: navigator.hardwareConcurrency,
    autosave: 5,
    start: 0,
    end: 1e8,
    current: 0,
    rules: [],
}

function validateRules(rules: SimpleRule[][]): boolean {
    if (rules.length === 0) return false
    for (const group of rules) {
        if (group.length === 0) return false
        for (const rule of group) {
            if (rule.type === RuleType.None) return false
            // TODO: Validate individual rule here
        }
    }
    return true
}

const Find: Component = () => {
    const [name, setName] = createSignal("")
    const [profile, setProfile] = createSignal<ProfileInfo | null>()
    const [progress, setProgress] = createStore<ProfileProgress>({
        ...defaultProgress,
    })
    const [nativeMode, setNativeMode] = createSignal(false)
    const [profileModal, setProfileModal] = createSignal(false)
    const [deleteModal, setDeleteModal] = createSignal(false)
    const [newModal, setNewModal] = createSignal(false)
    const [searching, setSearching] = createSignal(false)
    const isLoaded = () => !!profile()
    const hasProgress = () => progress.current > progress.start
    const isDisabled = () => searching() || hasProgress()
    const hasCompleted = () => progress.current >= progress.end

    function onSelectProfile(profile: ProfileInfo, progress: ProfileProgress) {
        batch(() => {
            setProfile(profile)
            setName(profile.name)
            setProgress(progress)
            setProfileModal(false)
        })
    }

    function onNewProfile() {
        batch(() => {
            setProgress({ ...defaultProgress })
            setProfile(null)
            setName("")
            setNewModal(false)
        })
    }

    function onCloneProfile() {
        batch(() => {
            setProgress({ id: "", current: 0 })
            setProfile(null)
            setName(name() + " - Copy")
        })
    }

    const isRuleValid = createMemo(() => validateRules(progress.rules))

    function isValid(): boolean {
        if (
            name() === "" ||
            progress.start < 0 ||
            progress.end > 1e8 ||
            progress.start >= progress.end ||
            progress.starCount < minStarCount ||
            progress.starCount > maxStarCount ||
            !Number.isInteger(progress.concurrency) ||
            progress.concurrency < 1 ||
            progress.autosave <= 0
        ) {
            return false
        }
        return isRuleValid()
    }

    async function onSaveProfile() {
        const existingProfile = profile()
        if (existingProfile) {
            await setProfileProgress(unwrap(progress))
            if (existingProfile.name !== name()) {
                const newProfile: ProfileInfo = {
                    ...existingProfile,
                    name: name(),
                }
                await setProfileInfo(newProfile)
                setProfile(newProfile)
            }
        } else {
            const id = generateProfileId()
            const newProfile: ProfileInfo = {
                id,
                name: name(),
                createdAt: Date.now(),
            }
            await setProfileInfo(newProfile)
            const newProgress: ProfileProgress = { ...unwrap(progress), id }
            await setProfileProgress(newProgress)
            batch(() => {
                setProgress(newProgress)
                setProfile(newProfile)
            })
            return
        }
    }

    async function onDeleteProfile() {
        const existingProfile = profile()
        if (existingProfile) {
            await deleteProfile(existingProfile.id)
        }
        batch(() => {
            onNewProfile()
            setDeleteModal(false)
        })
    }

    function onStartSearching() {
        setSearching(true)
        let results: FindResult[] = []
        getWorldGen(nativeMode()).find({
            gameDesc: {
                resourceMultiplier: progress.resourceMultiplier,
                starCount: progress.starCount,
            },
            range: [Math.max(progress.start, progress.current), progress.end],
            concurrency: progress.concurrency,
            autosave: progress.autosave,
            rule: constructRule(unwrap(progress.rules)),
            onResult: (result) => {
                results.push(result)
            },
            onProgress: (current) => {
                setProfileProgress(
                    {
                        ...unwrap(progress),
                        current,
                    },
                    results,
                ).then(() => {
                    setProgress("current", (c) => Math.max(c, current))
                })
                results = []
            },
            onError: (err) => {
                console.error(err)
                setSearching(false)
            },
            onComplete: () => {
                console.log("done")
                setSearching(false)
            },
            onInterrupt: () => {
                console.log("interrupt")
                setSearching(false)
            },
        })
    }

    function onStopSearching() {
        getWorldGen(nativeMode()).stop()
    }

    return (
        <div class={styles.content}>
            <div class={styles.top}>
                Profile:
                <Button
                    onClick={() => setProfileModal(true)}
                    disabled={searching()}
                >
                    Load
                </Button>
                <Button
                    onClick={onSaveProfile}
                    disabled={!isValid() || searching()}
                >
                    Save
                </Button>
                <Show when={isLoaded()}>
                    <Button
                        onClick={() => setNewModal(true)}
                        disabled={searching()}
                    >
                        New
                    </Button>
                    <Button onClick={onCloneProfile} disabled={searching()}>
                        Clone
                    </Button>
                    <Button
                        theme="error"
                        onClick={() => setDeleteModal(true)}
                        disabled={searching()}
                    >
                        Delete
                    </Button>
                </Show>
            </div>
            <div class={styles.fields}>
                <div class={styles.field}>
                    <div class={styles.label}>
                        {isLoaded() ? "" : "New "}Profile Name
                    </div>
                    <div class={styles.input}>
                        <Input
                            value={name()}
                            onChange={setName}
                            error={name() === ""}
                            disabled={searching()}
                        />
                    </div>
                </div>
                <div />
                <div class={styles.field}>
                    <div class={styles.label}>Star count</div>
                    <div class={styles.input}>
                        <StarCountSelector
                            class={styles.inputStandard}
                            value={progress.starCount}
                            onChange={(value) =>
                                setProgress("starCount", value)
                            }
                            disabled={isDisabled()}
                        />
                    </div>
                </div>
                <div class={styles.field}>
                    <div class={styles.label}>
                        <Tooltip text="To run the search in native mode, click the download button and run the program on your PC.">
                            Native Mode
                        </Tooltip>
                    </div>
                    <div class={styles.input}>
                        <Toggle
                            value={nativeMode()}
                            onChange={setNativeMode}
                            disabled={searching()}
                        />
                        <Button kind="outline" disabled={searching()}>
                            Download
                        </Button>
                    </div>
                </div>
                <div class={styles.field}>
                    <div class={styles.label}>Resource Multipler</div>
                    <div class={styles.input}>
                        <ResourceMultiplierSelector
                            class={styles.inputStandard}
                            value={progress.resourceMultiplier}
                            onChange={(value) =>
                                setProgress("resourceMultiplier", value)
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
                            value={progress.concurrency}
                            onChange={(value) =>
                                setProgress("concurrency", value)
                            }
                            emptyValue={-1}
                            maxLength={2}
                            error={
                                !Number.isInteger(progress.concurrency) ||
                                progress.concurrency < 1
                            }
                            disabled={searching()}
                        />
                    </div>
                </div>
                <div class={styles.field}>
                    <div class={styles.label}>Seed Range</div>
                    <div class={styles.input}>
                        <NumberInput
                            class={styles.inputSeed}
                            value={progress.start}
                            onChange={(value) => setProgress("start", value)}
                            emptyValue={-1}
                            maxLength={8}
                            error={
                                progress.start < 0 ||
                                progress.start >= progress.end
                            }
                            disabled={isDisabled()}
                        />{" "}
                        to{" "}
                        <NumberInput
                            class={styles.inputSeed}
                            value={progress.end - 1}
                            onChange={(value) => setProgress("end", value + 1)}
                            emptyValue={-1}
                            maxLength={8}
                            error={
                                progress.end > 1e8 ||
                                progress.start >= progress.end
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
                            value={progress.autosave}
                            onChange={(value) => setProgress("autosave", value)}
                            emptyValue={-1}
                            maxLength={2}
                            error={progress.autosave <= 0}
                            disabled={searching()}
                        />{" "}
                        seconds
                    </div>
                </div>
            </div>
            <div class={styles.rules}>Rules</div>
            <RuleEditor
                value={progress.rules}
                onChange={(rules) => setProgress("rules", rules)}
                disabled={isDisabled()}
            />
            <div class={styles.execute}>
                <div class={styles.progress}>
                    <Show
                        when={searching() || (hasProgress() && !hasCompleted())}
                    >
                        <div class={styles.progressText}>Progress:</div>
                        <ProgressBar
                            class={styles.progressBar}
                            current={progress.current}
                            total={progress.end - progress.start}
                        />
                    </Show>
                </div>
                <Switch
                    fallback={
                        <Button
                            disabled={!isValid()}
                            onClick={onStartSearching}
                        >
                            {hasProgress() ? "Resume" : "Start"}
                        </Button>
                    }
                >
                    <Match when={searching()}>
                        <Button onClick={onStopSearching}>Pause</Button>
                    </Match>
                    <Match when={hasCompleted()}>
                        <span class={styles.completed}>Completed!</span>
                    </Match>
                </Switch>
            </div>
            <ProfilesModal
                visible={profileModal()}
                onClose={() => setProfileModal(false)}
                onSelect={onSelectProfile}
            />
            <Modal
                visible={deleteModal()}
                onClose={() => setDeleteModal(false)}
            >
                <div class={styles.modalTitle}>Are you sure?</div>
                <div class={styles.warnText}>
                    Do you really want to delete all settings and progress? This
                    cannot be undone.
                </div>
                <div class={styles.warnButtons}>
                    <Button theme="error" onClick={onDeleteProfile}>
                        Delete
                    </Button>
                    <Button
                        kind="outline"
                        onClick={() => setDeleteModal(false)}
                    >
                        Cancel
                    </Button>
                </div>
            </Modal>
            <Modal visible={newModal()} onClose={() => setNewModal(false)}>
                <div class={styles.modalTitle}>Are you sure?</div>
                <div class={styles.warnText}>
                    Do you really want to create a new profile? All unsaved
                    changes will be lost.
                </div>
                <div class={styles.warnButtons}>
                    <Button onClick={onNewProfile}>Confirm</Button>
                    <Button kind="outline" onClick={() => setNewModal(false)}>
                        Cancel
                    </Button>
                </div>
            </Modal>
        </div>
    )
}

export default Find
