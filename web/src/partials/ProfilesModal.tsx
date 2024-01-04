import { For, JSX, Show, createEffect, createSignal } from "solid-js"
import Modal from "../components/Modal"
import styles from "./ProfilesModal.module.css"

function ProfilesModal(props: {
    visible: boolean
    onClose: () => void
    onSelect: (profile: ProfileInfo) => void
    loadProfiles: () => Promise<ProfileInfo[]>
}): JSX.Element {
    const [profiles, setProfiles] = createSignal<ProfileInfo[]>()
    createEffect(() => {
        if (props.visible) {
            props.loadProfiles().then(setProfiles)
        }
    })

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
                                onClick={() => props.onSelect(profile)}
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
