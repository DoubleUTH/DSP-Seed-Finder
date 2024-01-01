import { Component, For, Show, createEffect, createSignal } from "solid-js"
import { getProfileProgress, listProfiles } from "../profile"
import Modal from "../components/Modal"
import styles from "./ProfilesModal.module.css"

const ProfilesModal: Component<{
    visible: boolean
    onClose: () => void
    onSelect: (profile: ProfileInfo, progress: ProfileProgress) => void
}> = (props) => {
    const [profiles, setProfiles] = createSignal<ProfileInfo[]>()
    createEffect(() => {
        if (props.visible) {
            listProfiles().then(setProfiles)
        }
    })

    async function onSelect(profile: ProfileInfo) {
        const progress = await getProfileProgress(profile.id)
        if (progress) {
            props.onSelect(profile, progress)
        }
    }

    return (
        <Modal visible={props.visible} onClose={props.onClose}>
            <div class={styles.title}>Profiles</div>
            <div class={styles.profiles}>
                <Show
                    when={!!profiles()?.length}
                    fallback={
                        <div class={styles.noResult}>No saved profiles.</div>
                    }
                >
                    <For each={profiles()}>
                        {(profile) => (
                            <div
                                class={styles.profile}
                                onClick={() => onSelect(profile)}
                            >
                                <span class={styles.name}>{profile.name}</span>
                                <span class={styles.time}>
                                    {new Date(
                                        profile.createdAt,
                                    ).toLocaleString()}
                                </span>
                            </div>
                        )}
                    </For>
                </Show>
            </div>
        </Modal>
    )
}

export default ProfilesModal
