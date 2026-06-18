import { A, useNavigate, useParams } from "@solidjs/router"
import {
    Component,
    createSignal,
    Show,
    Switch,
    Match,
    batch,
    createMemo,
    createEffect,
    on,
    Index,
} from "solid-js"
import { createStore, unwrap } from "solid-js/store"
import {
    constructMultiRule,
    getDefaultParams,
    getSearch,
    maxStarCount,
    minStarCount,
    validateMultiRule,
} from "../util"
import {
    clearMultiProfile,
    deleteMultiProfile,
    generateProfileId,
    getMultiProfileInfo,
    getMultiProfileProgress,
    getMultiProfileResult,
    listMultiProfiles,
    setMultiProfileInfo,
    setMultiProfileProgress,
} from "../profile"
import { startSearchingGalaxies, stopSearchingGalaxies } from "../worldgen"
import ProgressEditor from "../partials/ProgressEditor"
import ProfileManager from "../partials/ProfileManager"
import styles from "./FindGalaxy.module.css"
import ProgressBar from "../components/ProgressBar"
import Button from "../components/Button"
import Pagination from "../components/Pagination"
import ProfilesModal from "../partials/ProfilesModal"
import MultiRuleEditor from "../partials/MultiRuleEditor"
import { ConditionType } from "../enums"
import { useStore } from "../store"
import ExportModal from "../partials/ExportModal"
import { useLingui } from "#lingui"
import { DEFAULT_BATCH_SIZE } from "../constants"

const PAGE_SIZE = 100

const defaultProgress: () => MultiProfileProgress = () => ({
    id: "",
    params: getDefaultParams(),
    concurrency: navigator.hardwareConcurrency,
    autosave: 5,
    range: [0, 1e8],
    total: 0,
    found: 0,
    batchSize: DEFAULT_BATCH_SIZE,
    nextBatchId: 0,
    multiRules: [
        [
            {
                rules: [],
                condition: { type: ConditionType.Gte, value: 1 },
                name: "",
            },
        ],
    ],
})

const SearchResult: Component<{
    id: string
    page: integer
    updateKey: number
    params: GameParameters
}> = (props) => {
    const [results, setResults] = createSignal<MultiProgressResult[]>([])
    let isLoading = -1

    const searchString = createMemo(() => getSearch(props.params))

    function update() {
        if (isLoading === props.page) return
        const page = props.page
        isLoading = page
        console.debug("results loading")
        getMultiProfileResult(props.id, (page - 1) * PAGE_SIZE, PAGE_SIZE).then(
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

    function buildUrl(item: MultiProgressResult) {
        return `/galaxy/${item.seed}${searchString()}`
    }

    return (
        <div class={styles.results}>
            <Index each={results()}>
                {(result) => (
                    <A
                        href={buildUrl(result())}
                        target="_blank"
                        class={styles.result}
                    >
                        {String(result().seed).padStart(8, "0")}
                    </A>
                )}
            </Index>
        </div>
    )
}

const FindGalaxy: Component = () => {
    const params = useParams()
    const navigate = useNavigate()
    const { t } = useLingui()
    const [name, setName] = createSignal(t`Untitled`)
    const [profile, setProfile] = createSignal<ProfileInfo | null>()
    const [progress, setProgress] =
        createStore<MultiProfileProgress>(defaultProgress())
    const [nativeMode, setNativeMode] = createSignal(false)
    const [profileModal, setProfileModal] = createSignal(false)
    const [exportModal, setExportModal] = createSignal(false)
    const [store, setStore] = useStore()
    const [currentPage, setCurrentPage] = createSignal(1)
    const [tick, setTick] = createSignal(0)
    const isLoaded = () => !!profile()
    const hasProgress = () => progress.nextBatchId > 0
    const isDisabled = () => store.searching || hasProgress()
    const hasCompleted = () => {
        const totalBatchCount = Math.ceil(progress.total / progress.batchSize)
        return totalBatchCount > 0 && progress.nextBatchId >= totalBatchCount
    }

    function changeProfile(profile: ProfileInfo | null) {
        batch(() => {
            if (profile) {
                navigate(`/find-galaxy/${profile.id}`)
                setProfile(profile)
                setName(profile.name)
            } else {
                navigate(`/find-galaxy`)
                setProfile(null)
                setName("")
            }
        })
    }

    async function onSelectProfile(profile: ProfileInfo) {
        const progress = await getMultiProfileProgress(profile.id)
        if (progress) {
            console.debug(progress)
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
            setName(origName + t` - Copy`)
            setProgress({
                id: "",
                batchSize: DEFAULT_BATCH_SIZE,
                nextBatchId: 0,
                found: 0,
            })
        })
    }

    const isRuleValid = createMemo(() => validateMultiRule(progress.multiRules))

    function isValid(): boolean {
        if (
            name() === "" ||
            progress.params.starCount < minStarCount ||
            progress.params.starCount > maxStarCount ||
            !Number.isInteger(progress.concurrency) ||
            progress.concurrency < 1 ||
            progress.autosave <= 0
        ) {
            return false
        }
        if (Array.isArray(progress.range)) {
            if (
                progress.range[0] < 0 ||
                progress.range[1] > 1e8 ||
                progress.range[0] >= progress.range[1]
            ) {
                return false
            }
        }
        return isRuleValid()
    }

    async function onSaveProfile() {
        const existingProfile = profile()
        if (existingProfile) {
            await setMultiProfileProgress(unwrap(progress))
            if (existingProfile.name !== name()) {
                const newProfile: ProfileInfo = {
                    ...existingProfile,
                    name: name(),
                }
                await setMultiProfileInfo(newProfile)
                changeProfile(newProfile)
            }
        } else {
            const id = generateProfileId()
            const newProfile: ProfileInfo = {
                id,
                name: name(),
                createdAt: Date.now(),
            }
            await setMultiProfileInfo(newProfile)
            const newProgress: MultiProfileProgress = {
                ...unwrap(progress),
                id,
            }
            await setMultiProfileProgress(newProgress)
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
            await clearMultiProfile(existingProfile.id)
        }
        batch(() => {
            setCurrentPage(1)
            setProgress({
                found: 0,
                nextBatchId: 0,
                batchSize: DEFAULT_BATCH_SIZE,
            })
        })
    }

    async function onDeleteProfile() {
        const existingProfile = profile()
        if (existingProfile) {
            await deleteMultiProfile(existingProfile.id)
        }
        batch(() => {
            onNewProfile()
        })
    }

    async function onStartSearching() {
        await onSaveProfile()
        setStore("searching", true)
        let results: integer[] = []
        setProgress({
            total: Array.isArray(progress.range)
                ? progress.range[1] - progress.range[0]
                : progress.range.length,
        })
        startSearchingGalaxies(nativeMode(), {
            batchSize: progress.batchSize,
            nextBatchId: progress.nextBatchId,
            gameDesc: progress.params,
            range: progress.range,
            concurrency: progress.concurrency,
            autosave: progress.autosave,
            rule: constructMultiRule(unwrap(progress.multiRules)),
            onResult: (result) => {
                console.debug("result", result)
                results.push(...result)
            },
            onProgress: (nextBatchId) => {
                batch(() => {
                    setProgress("nextBatchId", (c) => Math.max(c, nextBatchId))
                    setProgress("found", (found) => found + results.length)
                })
                setMultiProfileProgress(unwrap(progress), results).then(() => {
                    setTick((prev) => (prev + 1) % 1024)
                })
                results = []
            },
            onError: (err) => {
                console.error(err)
                setStore("searching", false)
            },
            onComplete: () => {
                console.debug("done")
                setStore("searching", false)
            },
            onInterrupt: () => {
                console.debug("interrupt")
                setStore("searching", false)
            },
        })
    }

    function onStopSearching() {
        stopSearchingGalaxies(nativeMode())
    }

    createEffect(
        on(
            () => params.profileId,
            (profileId) => {
                if (profileId) {
                    if (profile()?.id !== profileId) {
                        Promise.all([
                            getMultiProfileInfo(profileId),
                            getMultiProfileProgress(profileId),
                        ]).then(([info, progress]): void => {
                            if (info && info.id === profileId) {
                                batch(() => {
                                    setName(info.name)
                                    setProfile(info)
                                    setName(info.name)
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
                disabled={store.searching}
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
                searching={store.searching}
            />
            <div class={styles.rules}>{t`Rules`}</div>
            <MultiRuleEditor
                value={progress.multiRules}
                onChange={(multiRules) => setProgress("multiRules", multiRules)}
                disabled={isDisabled()}
            />
            <div class={styles.execute}>
                <div class={styles.progress}>
                    <Show
                        when={
                            store.searching ||
                            (hasProgress() && !hasCompleted())
                        }
                    >
                        <div class={styles.progressText}>{t`Progress:`}</div>
                        <ProgressBar
                            class={styles.progressBar}
                            current={progress.nextBatchId * progress.batchSize}
                            total={progress.total}
                        />
                    </Show>
                </div>
                <Show when={hasProgress()}>
                    <Button
                        onClick={() => setExportModal(true)}
                    >{t`Export`}</Button>
                </Show>
                <Switch
                    fallback={
                        <Button
                            disabled={!isValid()}
                            onClick={onStartSearching}
                        >
                            {hasProgress() ? t`Resume` : t`Start`}
                        </Button>
                    }
                >
                    <Match when={store.searching}>
                        <Button onClick={onStopSearching}>{t`Pause`}</Button>
                    </Match>
                    <Match when={hasCompleted()}>
                        <span class={styles.completed}>{t`Completed!`}</span>
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
                    params={progress.params}
                />
            </Show>
            <ProfilesModal
                visible={profileModal()}
                onClose={() => setProfileModal(false)}
                onSelect={onSelectProfile}
                loadProfiles={listMultiProfiles}
            />
            <ExportModal
                visible={exportModal()}
                onClose={() => setExportModal(false)}
                mode="galaxy"
                id={profile()?.id || ""}
                name={profile()?.name || ""}
                params={progress.params}
            />
        </div>
    )
}

export default FindGalaxy
