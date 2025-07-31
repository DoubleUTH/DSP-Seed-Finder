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
import { getWorldGen } from "../worldgen"
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

const PAGE_SIZE = 100

const defaultProgress: () => MultiProfileProgress = () => ({
    id: "",
    starCount: 64,
    resourceMultiplier: 1,
    concurrency: navigator.hardwareConcurrency,
    autosave: 5,
    start: 0,
    end: 1e8,
    current: 0,
    found: 0,
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
    results: any[]
    starCount: integer
    resourceMultiplier: float
}> = (props) => {
    const searchString = createMemo(() =>
        getSearch({
            count: props.starCount,
            multipler: props.resourceMultiplier,
        }),
    )

    function buildUrl(item: any) {
        return `/galaxy/${item.seed}/0${searchString()}`
    }

    return (
        <div class={styles.results}>
            <Index each={props.results}>
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
    const [name, setName] = createSignal("Untitled")
    const [profile, setProfile] = createSignal<ProfileInfo | null>()
    const [progress, setProgress] =
        createStore<MultiProfileProgress>(defaultProgress())
    const [profileModal, setProfileModal] = createSignal(false)
    const [exportModal, setExportModal] = createSignal(false)
    const [store, setStore] = useStore()
    const [currentPage, setCurrentPage] = createSignal(1)
    const [tick, setTick] = createSignal(0)
    const [filteredData, setFilteredData] = createSignal<any[]>([])
    const isLoaded = () => !!profile()
    const hasProgress = () =>
        progress.start > -1 && progress.current > progress.start
    const isDisabled = () => store.searching || hasProgress()
    const hasCompleted = () =>
        progress.start > -1 && progress.current >= progress.end

    const [galaxyData, setGalaxyData] = createSignal<any[]>([])

    createEffect(() => {
        fetch("/galaxy_data.jsonl")
            .then((response) => response.text())
            .then((text) => {
                const lines = text.trim().split("\n")
                const data = lines.map((line) => JSON.parse(line))
                setGalaxyData(data)
            })
    })

    function onSearch() {
        const rules = unwrap(progress.multiRules)
        const filteredData = galaxyData().filter((galaxy) => {
            // NOTE: This is a simplified search implementation.
            // A more robust implementation would involve a proper query engine.
            for (const ruleGroup of rules) {
                let groupMatch = true
                for (const rule of ruleGroup) {
                    let ruleMatch = false
                    // TODO: Implement rule matching logic
                    groupMatch = groupMatch && ruleMatch
                }
                if (groupMatch) {
                    return true
                }
            }
            return false
        })
        // TODO: Display the filtered data
        console.log("Filtered data:", filteredData)
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

    const isRuleValid = createMemo(() => validateMultiRule(progress.multiRules))

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
            setProgress("current", 0)
            setProgress("found", 0)
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
                disabled={store.searching}
                isValid={isValid()}
                isLoaded={isLoaded()}
            />
            <div class={styles.rules}>Rules</div>
            <MultiRuleEditor
                value={progress.multiRules}
                onChange={(multiRules) => setProgress("multiRules", multiRules)}
                disabled={isDisabled()}
            />
            <div class={styles.execute}>
                <Button onClick={onSearch}>Search</Button>
            </div>
            <Show when={filteredData().length > 0}>
                <SearchResult
                    results={filteredData()}
                    starCount={progress.starCount}
                    resourceMultiplier={progress.resourceMultiplier}
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
                starCount={progress.starCount}
                resourceMultiplier={progress.resourceMultiplier}
            />
        </div>
    )
}

export default FindGalaxy
