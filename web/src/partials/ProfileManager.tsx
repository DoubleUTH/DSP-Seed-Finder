import { Component, Show, createSignal } from "solid-js"
import styles from "~styles"
import Button from "../components/Button"
import Modal from "../components/Modal"
import { useLingui } from "#lingui"

const ProfileManager: Component<{
    onLoad: () => void
    onSave?: () => void
    onNew?: () => void
    onClone?: () => void
    onClear?: () => void
    onDelete?: () => void
    disabled: boolean
    isLoaded: boolean
    isValid: boolean
}> = (props) => {
    const [clearModal, setClearModal] = createSignal(false)
    const [deleteModal, setDeleteModal] = createSignal(false)
    const [newModal, setNewModal] = createSignal(false)

    const { t } = useLingui()

    return (
        <div class={styles.top}>
            {t`Profile:`}
            <Button onClick={props.onLoad} disabled={props.disabled}>
                {t`Load`}
            </Button>
            <Show when={props.onSave}>
                <Button
                    onClick={props.onSave}
                    disabled={props.disabled || !props.isValid}
                >
                    {t`Save`}
                </Button>
            </Show>
            <Show when={props.isLoaded}>
                <Show when={props.onNew}>
                    <Button
                        onClick={() => setNewModal(true)}
                        disabled={props.disabled}
                    >
                        {t`New`}
                    </Button>
                </Show>
                <Show when={props.onClone}>
                    <Button onClick={props.onClone} disabled={props.disabled}>
                        {t`Clone`}
                    </Button>
                </Show>
                <Show when={props.onClear}>
                    <Button
                        theme="error"
                        onClick={() => setClearModal(true)}
                        disabled={props.disabled}
                    >
                        {t`Clear`}
                    </Button>
                </Show>
                <Show when={props.onDelete}>
                    <Button
                        theme="error"
                        onClick={() => setDeleteModal(true)}
                        disabled={props.disabled}
                    >
                        {t`Delete`}
                    </Button>
                </Show>
            </Show>
            <Modal
                visible={clearModal()}
                onClose={() => setClearModal(false)}
                backdropDismiss
            >
                <div class={styles.modalTitle}>{t`Are you sure?`}</div>
                <div class={styles.warnText}>
                    {t`Do you really want to clear all progress? This cannot be undone.`}
                </div>
                <div class={styles.warnButtons}>
                    <Button
                        theme="error"
                        onClick={() => {
                            setClearModal(false)
                            props.onClear?.()
                        }}
                    >
                        {t`Clear`}
                    </Button>
                    <Button kind="outline" onClick={() => setClearModal(false)}>
                        {t`Cancel`}
                    </Button>
                </div>
            </Modal>
            <Modal
                visible={deleteModal()}
                onClose={() => setDeleteModal(false)}
                backdropDismiss
            >
                <div class={styles.modalTitle}>{t`Are you sure?`}</div>
                <div class={styles.warnText}>
                    {t`Do you really want to delete all settings and progress? This cannot be undone.`}
                </div>
                <div class={styles.warnButtons}>
                    <Button
                        theme="error"
                        onClick={() => {
                            setDeleteModal(false)
                            props.onDelete?.()
                        }}
                    >
                        {t`Delete`}
                    </Button>
                    <Button
                        kind="outline"
                        onClick={() => setDeleteModal(false)}
                    >
                        {t`Cancel`}
                    </Button>
                </div>
            </Modal>
            <Modal
                visible={newModal()}
                onClose={() => setNewModal(false)}
                backdropDismiss
            >
                <div class={styles.modalTitle}>{t`Are you sure?`}</div>
                <div class={styles.warnText}>
                    {t`Do you really want to create a new profile? All unsaved changes will be lost.`}
                </div>
                <div class={styles.warnButtons}>
                    <Button
                        onClick={() => {
                            setNewModal(false)
                            props.onNew?.()
                        }}
                    >
                        {t`Confirm`}
                    </Button>
                    <Button kind="outline" onClick={() => setNewModal(false)}>
                        {t`Cancel`}
                    </Button>
                </div>
            </Modal>
        </div>
    )
}

export default ProfileManager
