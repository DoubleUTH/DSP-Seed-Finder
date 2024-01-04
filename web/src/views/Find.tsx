import {
    Component,
    Index,
    Match,
    Show,
    Switch,
    batch,
    createEffect,
    createMemo,
    createSignal,
    on,
} from "solid-js"
import Button from "../components/Button"
import { getWorldGen } from "../worldgen"
import { RuleType } from "../enums"
import {
    clearProfile,
    deleteProfile,
    generateProfileId,
    getProfileInfo,
    getProfileProgress,
    getProfileResult,
    setProfileInfo,
    setProfileProgress,
} from "../profile"
import { constructRule, getSearch, maxStarCount, minStarCount } from "../util"
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
import { IoChevronBack, IoChevronForward, IoOpenOutline } from "solid-icons/io"
import StarView from "../partials/StarView"
import ExeUrl from "../../../target/release/dsp_seed.exe?url"
import { A, useNavigate, useParams } from "@solidjs/router"

const defaultProgress: ProfileProgress = {
    id: "",
    starCount: 64,
    resourceMultiplier: 1,
    concurrency: navigator.hardwareConcurrency,
    autosave: 5,
    start: 0,
    end: 1e8,
    current: 0,
    found: 0,
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

const PAGE_SIZE = 100

const Pagination: Component<{
    current: integer
    total: integer
    onChange: (page: integer) => void
}> = (props) => {
    // eslint-disable-next-line solid/reactivity
    const [page, setPage] = createSignal(props.current)

    function onChange(value: number) {
        if (value >= 1 && value <= props.total) {
            props.onChange(value)
            setPage(value)
        }
    }

    function handleSubmit(ev: Event) {
        ev.preventDefault()
        onChange(page())
    }

    return (
        <form class={styles.pagination} onSubmit={handleSubmit}>
            <button
                type="button"
                class={styles.paginationButton}
                disabled={props.current <= 1}
                onClick={() => onChange(props.current - 1)}
            >
                <IoChevronBack />
            </button>
            Page{" "}
            <NumberInput
                class={styles.paginationInput}
                value={page()}
                onChange={setPage}
                onBlur={() => onChange(page())}
                emptyValue={-1}
                error={
                    !Number.isInteger(page()) ||
                    page() <= 0 ||
                    page() > props.total
                }
            />{" "}
            of {props.total}{" "}
            <button
                type="button"
                class={styles.paginationButton}
                disabled={props.current >= props.total}
                onClick={() => onChange(props.current + 1)}
            >
                <IoChevronForward />
            </button>
        </form>
    )
}

const StarViewModal: Component<{
    seed: integer
    index: integer
    starCount: integer
    resourceMultiplier: float
    search: string
}> = (props) => {
    const [galaxy, setGalaxy] = createSignal<Galaxy | null>(null)

    createEffect(() => {
        getWorldGen(false)
            .generate({
                seed: props.seed,
                starCount: props.starCount,
                resourceMultiplier: props.resourceMultiplier,
            })
            .then((g): void => {
                setGalaxy(g)
            })
    })

    function buildUrl(starIndex: integer) {
        return `/galaxy/${props.seed}/${starIndex}${props.search}`
    }

    return (
        <Show when={!!galaxy()}>
            <div class={styles.viewTop}>
                <div class={styles.viewTitle}>
                    Seed: {String(props.seed).padStart(8, "0")}
                </div>
                <A
                    class={styles.viewNewTab}
                    href={buildUrl(props.index)}
                    target="_blank"
                >
                    View in new tab
                    <IoOpenOutline />
                </A>
            </div>
            <StarView
                star={galaxy()!.stars[props.index]!}
                galaxy={galaxy()!}
                buildUrl={buildUrl}
                newPage
            />
        </Show>
    )
}

const SearchResult: Component<{
    id: string
    page: integer
    updateKey: number
    starCount: integer
    resourceMultiplier: float
}> = (props) => {
    const [results, setResults] = createSignal<ProgressResult[]>([])
    const [active, setActive] = createSignal<ProgressResult | null>(null)
    let isLoading = -1

    const searchString = createMemo(() =>
        getSearch({
            count: props.starCount,
            multipler: props.resourceMultiplier,
        }),
    )

    function update() {
        if (isLoading === props.page) return
        const page = props.page
        isLoading = page
        console.debug("results loading")
        getProfileResult(props.id, (page - 1) * PAGE_SIZE, PAGE_SIZE).then(
            (list) => {
                console.debug("results loaded", list)
                if (isLoading === page) {
                    setResults(list)
                    isLoading = -1
                }
            },
        )
    }

    createEffect(update)

    createEffect(
        on(
            () => props.updateKey,
            () => {
                if (results().length < PAGE_SIZE) {
                    update()
                }
            },
        ),
    )

    function buildUrl(item: ProgressResult) {
        return `/galaxy/${item.seed}/${item.index}${searchString()}`
    }

    return (
        <>
            <div class={styles.results}>
                <Index each={results()}>
                    {(result) => (
                        <A
                            href={buildUrl(result())}
                            target="_blank"
                            class={styles.result}
                            onClick={(ev) => {
                                ev.preventDefault()
                                setActive(result())
                            }}
                        >
                            <span class={styles.resultSeed}>
                                {String(result().seed).padStart(8, "0")}
                            </span>
                            <span class={styles.resultIndex}>
                                #{result().index + 1}
                            </span>
                        </A>
                    )}
                </Index>
            </div>
            <Show when={!!active()}>
                <Modal visible onClose={() => setActive(null)}>
                    <StarViewModal
                        seed={active()!.seed}
                        index={active()!.index}
                        starCount={props.starCount}
                        resourceMultiplier={props.resourceMultiplier}
                        search={searchString()}
                    />
                </Modal>
            </Show>
        </>
    )
}

const Find: Component = () => {
    const params = useParams()
    const navigate = useNavigate()
    const [name, setName] = createSignal("Untitled")
    const [profile, setProfile] = createSignal<ProfileInfo | null>()
    const [progress, setProgress] = createStore<ProfileProgress>({
        ...defaultProgress,
    })
    const [nativeMode, setNativeMode] = createSignal(false)
    const [profileModal, setProfileModal] = createSignal(false)
    const [clearModal, setClearModal] = createSignal(false)
    const [deleteModal, setDeleteModal] = createSignal(false)
    const [newModal, setNewModal] = createSignal(false)
    const [searching, setSearching] = createSignal(false)
    const [currentPage, setCurrentPage] = createSignal(1)
    const [tick, setTick] = createSignal(0)
    const isLoaded = () => !!profile()
    const hasProgress = () =>
        progress.start > -1 && progress.current > progress.start
    const isDisabled = () => searching() || hasProgress()
    const hasCompleted = () =>
        progress.start > -1 && progress.current >= progress.end

    function changeProfile(profile: ProfileInfo | null) {
        batch(() => {
            if (profile) {
                navigate(`/${profile.id}`)
                setProfile(profile)
                setName(profile.name)
            } else {
                navigate(`/`)
                setProfile(null)
                setName("")
            }
        })
    }

    function onSelectProfile(profile: ProfileInfo, progress: ProfileProgress) {
        batch(() => {
            changeProfile(profile)
            setProgress(progress)
            setProfileModal(false)
        })
    }

    function onNewProfile() {
        batch(() => {
            setProgress({ ...defaultProgress })
            changeProfile(null)
            setNewModal(false)
        })
    }

    function onCloneProfile() {
        batch(() => {
            setProgress({ id: "", current: 0 })
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
                changeProfile(newProfile)
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
                changeProfile(newProfile)
            })
            return
        }
    }

    async function onClearProfile() {
        const existingProfile = profile()
        if (existingProfile) {
            await clearProfile(existingProfile.id)
        }
        batch(() => {
            setCurrentPage(1)
            setProgress("current", 0)
            setProgress("found", 0)
            setClearModal(false)
        })
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

    async function onStartSearching() {
        await onSaveProfile()
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
                console.debug("result", result)
                results.push(result)
            },
            onProgress: (current) => {
                batch(() => {
                    setProgress("current", (c) => Math.max(c, current))
                    setProgress("found", (found) => found + results.length)
                })
                setProfileProgress(unwrap(progress), results).then(() => {
                    setTick((prev) => (prev + 1) % 1024)
                })
                results = []
            },
            onError: (err) => {
                console.error(err)
                setSearching(false)
            },
            onComplete: () => {
                console.debug("done")
                setSearching(false)
            },
            onInterrupt: () => {
                console.debug("interrupt")
                setSearching(false)
            },
        })
    }

    function onStopSearching() {
        getWorldGen(nativeMode()).stop()
    }

    createEffect(() => {
        const { profileId } = params
        if (profileId && profile()?.id !== profileId) {
            Promise.all([
                getProfileInfo(profileId),
                getProfileProgress(profileId),
            ]).then(([info, progress]): void => {
                if (info && info.id === profileId) {
                    setProfileInfo(info)
                }
                if (progress && progress.id === profileId) {
                    setProgress(progress)
                }
            })
        }
    })

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
                        onClick={() => setClearModal(true)}
                        disabled={searching()}
                    >
                        Clear
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
                        <Tooltip text="To run the search in (faster) native mode, click the download button and run the program on your PC, then enable this option.">
                            Native Mode
                        </Tooltip>
                    </div>
                    <div class={styles.input}>
                        <Toggle
                            value={nativeMode()}
                            onChange={setNativeMode}
                            disabled={searching()}
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
                            current={progress.current - progress.start}
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
            <Show when={hasProgress()}>
                <Pagination
                    current={currentPage()}
                    total={
                        Math.max(
                            0,
                            Math.floor((progress.found - 1) / PAGE_SIZE),
                        ) + 1
                    }
                    onChange={setCurrentPage}
                />
                <SearchResult
                    id={profile()!.id}
                    page={currentPage()}
                    updateKey={tick()}
                    starCount={progress.starCount}
                    resourceMultiplier={progress.resourceMultiplier}
                />
            </Show>
            <ProfilesModal
                visible={profileModal()}
                onClose={() => setProfileModal(false)}
                onSelect={onSelectProfile}
            />
            <Modal visible={clearModal()} onClose={() => setClearModal(false)}>
                <div class={styles.modalTitle}>Are you sure?</div>
                <div class={styles.warnText}>
                    Do you really want to clear all progress? This cannot be
                    undone.
                </div>
                <div class={styles.warnButtons}>
                    <Button theme="error" onClick={onClearProfile}>
                        Clear
                    </Button>
                    <Button kind="outline" onClick={() => setClearModal(false)}>
                        Cancel
                    </Button>
                </div>
            </Modal>
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
