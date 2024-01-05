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
import {
    clearProfile,
    deleteProfile,
    generateProfileId,
    getProfileInfo,
    getProfileProgress,
    getProfileResult,
    listProfiles,
    setProfileInfo,
    setProfileProgress,
} from "../profile"
import {
    constructRule,
    getSearch,
    maxStarCount,
    minStarCount,
    validateRules,
} from "../util"
import RuleEditor from "../partials/RuleEditor"
import styles from "./FindStar.module.css"
import { createStore, unwrap } from "solid-js/store"
import ProfilesModal from "../partials/ProfilesModal"
import Modal from "../components/Modal"
import ProgressBar from "../components/ProgressBar"
import { IoOpenOutline } from "solid-icons/io"
import StarView from "../partials/StarView"
import { A, useNavigate, useParams } from "@solidjs/router"
import ProgressEditor from "../partials/ProgressEditor"
import ProfileManager from "../partials/ProfileManager"
import Pagination from "../components/Pagination"

const defaultProgress: () => ProfileProgress = () => ({
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
})

const PAGE_SIZE = 100

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

const FindStar: Component = () => {
    const params = useParams()
    const navigate = useNavigate()
    const [name, setName] = createSignal("Untitled")
    const [profile, setProfile] = createSignal<ProfileInfo | null>()
    const [progress, setProgress] =
        createStore<ProfileProgress>(defaultProgress())
    const [nativeMode, setNativeMode] = createSignal(false)
    const [profileModal, setProfileModal] = createSignal(false)
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
                navigate(`/find-star/${profile.id}`)
                setProfile(profile)
                setName(profile.name)
            } else {
                navigate(`/find-star`)
                setProfile(null)
                setName("")
            }
        })
    }

    async function onSelectProfile(profile: ProfileInfo) {
        const progress = await getProfileProgress(profile.id)
        if (progress) {
            batch(() => {
                changeProfile(profile)
                setProgress(progress)
                setProfileModal(false)
            })
        }
    }

    function onNewProfile() {
        batch(() => {
            setProgress(defaultProgress())
            changeProfile(null)
        })
    }

    function onCloneProfile() {
        batch(() => {
            const origName = name()
            changeProfile(null)
            setName(origName + " - Copy")
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
        })
    }

    async function onDeleteProfile() {
        const existingProfile = profile()
        if (existingProfile) {
            await deleteProfile(existingProfile.id)
        }
        onNewProfile()
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

    createEffect(
        on(
            () => params.profileId,
            (profileId) => {
                if (profileId) {
                    if (profile()?.id !== profileId) {
                        Promise.all([
                            getProfileInfo(profileId),
                            getProfileProgress(profileId),
                        ]).then(([info, progress]): void => {
                            if (info && info.id === profileId) {
                                batch(() => {
                                    setProfile(info)
                                    if (progress && progress.id === profileId) {
                                        setProgress(progress)
                                    }
                                })
                            }
                        })
                    }
                }
            },
        ),
    )

    return (
        <div class={styles.content}>
            <ProfileManager
                onLoad={() => setProfileModal(true)}
                onSave={onSaveProfile}
                onNew={onNewProfile}
                onClone={onCloneProfile}
                onClear={onClearProfile}
                onDelete={onDeleteProfile}
                disabled={searching()}
                isValid={isValid()}
                isLoaded={isLoaded()}
            />
            <ProgressEditor
                progress={progress}
                onProgressChange={setProgress}
                name={name()}
                onNameChange={setName}
                nativeMode={nativeMode()}
                onNativeModeChange={setNativeMode}
                isLoaded={isLoaded()}
                searching={searching()}
            />
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
                loadProfiles={listProfiles}
            />
        </div>
    )
}

export default FindStar
