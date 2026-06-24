import {
    Component,
    Index,
    Show,
    batch,
    createEffect,
    createMemo,
    createSignal,
    on,
} from "solid-js"
import Button from "../components/Button"
import {
    getProfileInfo,
    getProfileProgress,
    getProfileResult,
    listProfiles,
} from "../profile"
import {
    getDefaultParams,
    getSearch,
    maxStarCount,
    minStarCount,
    validateRules,
} from "../util"
import RuleEditor from "../partials/RuleEditor"
import styles from "~styles"
import { createStore } from "solid-js/store"
import ProfilesModal from "../partials/ProfilesModal"
import Modal from "../components/Modal"
import ProgressBar from "../components/ProgressBar"
import { IoOpenOutline } from "solid-icons/io"
import StarView from "../partials/StarView"
import { A, useNavigate, useParams } from "@solidjs/router"
import ProgressEditor from "../partials/ProgressEditor"
import ProfileManager from "../partials/ProfileManager"
import Pagination from "../components/Pagination"
import { useStore } from "../store"
import ExportModal from "../partials/ExportModal"
import { useLingui } from "#lingui"
import { DEFAULT_BATCH_SIZE } from "../constants"
import { generateGalaxy } from "../worldgen"

const defaultProgress: () => ProfileProgress = () => ({
    id: "",
    params: getDefaultParams(),
    concurrency: navigator.hardwareConcurrency,
    autosave: 5,
    range: [0, 1e8],
    total: 0,
    found: 0,
    batchSize: DEFAULT_BATCH_SIZE,
    nextBatchId: 0,
    totalBatchCount: 0,
    rules: [],
})

const PAGE_SIZE = 100

const StarViewModal: Component<{
    seed: integer
    index: integer
    params: GameParameters
    search: string
}> = (props) => {
    const [galaxy, setGalaxy] = createSignal<Galaxy | null>(null)

    createEffect(() => {
        generateGalaxy(false, props.seed, props.params).then((g): void => {
            setGalaxy(g)
        })
    })

    function buildUrl(starIndex: integer) {
        return `/galaxy/${props.seed}/${starIndex}${props.search}`
    }

    const { t } = useLingui()

    return (
        <Show when={!!galaxy()}>
            <div class={styles.viewTop}>
                <div class={styles.viewTitle}>
                    {t`Seed: `}
                    {String(props.seed).padStart(8, "0")}
                </div>
                <A
                    class={styles.viewNewTab}
                    href={buildUrl(props.index)}
                    target="_blank"
                >
                    {t`View in new tab`}
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
    params: GameParameters
}> = (props) => {
    const [results, setResults] = createSignal<ProgressResult[]>([])
    const [active, setActive] = createSignal<ProgressResult | null>(null)
    let isLoading = -1

    const searchString = createMemo(() => getSearch(props.params))

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
                <Modal visible onClose={() => setActive(null)} backdropDismiss>
                    <StarViewModal
                        seed={active()!.seed}
                        index={active()!.index}
                        params={props.params}
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
    const { t } = useLingui()
    const [name, setName] = createSignal(t`Untitled`)
    const [profile, setProfile] = createSignal<ProfileInfo | null>()
    const [progress, setProgress] =
        createStore<ProfileProgress>(defaultProgress())
    const [nativeMode, setNativeMode] = createSignal(false)
    const [profileModal, setProfileModal] = createSignal(false)
    const [exportModal, setExportModal] = createSignal(false)
    const [store] = useStore()
    const [currentPage, setCurrentPage] = createSignal(1)
    const isLoaded = () => !!profile()
    const hasProgress = () => progress.nextBatchId > 0
    const isDisabled = () => true
    const hasCompleted = () => {
        const totalBatchCount = Math.ceil(progress.total / progress.batchSize)
        return totalBatchCount > 0 && progress.nextBatchId >= totalBatchCount
    }

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

    const isRuleValid = createMemo(() => validateRules(progress.rules))

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
            <div
                class={styles.warning}
            >{t`Star Finder is no longer supported. Please use Galaxy Finder instead.`}</div>
            <ProfileManager
                onLoad={() => setProfileModal(true)}
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
            <RuleEditor
                value={progress.rules}
                onChange={(rules) => setProgress("rules", rules)}
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
                    updateKey={0}
                    params={progress.params}
                />
            </Show>
            <ProfilesModal
                visible={profileModal()}
                onClose={() => setProfileModal(false)}
                onSelect={onSelectProfile}
                loadProfiles={listProfiles}
            />
            <ExportModal
                visible={exportModal()}
                onClose={() => setExportModal(false)}
                mode="star"
                id={profile()?.id || ""}
                name={profile()?.name || ""}
                params={progress.params}
            />
        </div>
    )
}

export default FindStar
