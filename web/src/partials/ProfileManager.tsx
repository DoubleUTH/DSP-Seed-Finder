import { Component, Show, createSignal } from "solid-js"
import styles from "./ProfileManager.module.css"
import Button from "../components/Button"
import Modal from "../components/Modal"

const ProfileManager: Component<{
    onLoad: () => void
    onSave: () => void
    onNew: () => void
    onClone: () => void
    onClear: () => void
    onDelete: () => void
    disabled: boolean
    isLoaded: boolean
    isValid: boolean
}> = (props) => {
    const [clearModal, setClearModal] = createSignal(false)
    const [deleteModal, setDeleteModal] = createSignal(false)
    const [newModal, setNewModal] = createSignal(false)

    return (
        <div class={styles.top}>
            Profile:
            <Button onClick={props.onLoad} disabled={props.disabled}>
                Load
            </Button>
            <Button
                onClick={props.onSave}
                disabled={props.disabled || !props.isValid}
            >
                Save
            </Button>
            <Show when={props.isLoaded}>
                <Button
                    onClick={() => setNewModal(true)}
                    disabled={props.disabled}
                >
                    New
                </Button>
                <Button onClick={props.onClone} disabled={props.disabled}>
                    Clone
                </Button>
                <Button
                    theme="error"
                    onClick={() => setClearModal(true)}
                    disabled={props.disabled}
                >
                    Clear
                </Button>
                <Button
                    theme="error"
                    onClick={() => setDeleteModal(true)}
                    disabled={props.disabled}
                >
                    Delete
                </Button>
            </Show>
            <Modal visible={clearModal()} onClose={() => setClearModal(false)}>
                <div class={styles.modalTitle}>Are you sure?</div>
                <div class={styles.warnText}>
                    Do you really want to clear all progress? This cannot be
                    undone.
                </div>
                <div class={styles.warnButtons}>
                    <Button
                        theme="error"
                        onClick={() => {
                            setClearModal(false)
                            props.onClear()
                        }}
                    >
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
                    <Button
                        theme="error"
                        onClick={() => {
                            setDeleteModal(false)
                            props.onDelete()
                        }}
                    >
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
                    <Button
                        onClick={() => {
                            setNewModal(false)
                            props.onNew()
                        }}
                    >
                        Confirm
                    </Button>
                    <Button kind="outline" onClick={() => setNewModal(false)}>
                        Cancel
                    </Button>
                </div>
            </Modal>
        </div>
    )
}

export default ProfileManager
