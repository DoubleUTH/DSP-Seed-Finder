import styles from "./Modal.module.css"
import { ParentComponent, Show } from "solid-js"
import clsx from "clsx"
import { IoClose } from "solid-icons/io"

const Modal: ParentComponent<{
    class?: string
    visible: boolean
    onClose?: () => void
    backdropDismiss?: boolean
}> = (props) => {
    function handleBackdrop(ev: MouseEvent) {
        if (props.backdropDismiss && ev.currentTarget === ev.target) {
            props.onClose?.()
        }
    }
    return (
        <div
            class={clsx(
                styles.modal,
                props.class,
                props.visible ? styles.visible : styles.hidden,
            )}
            onClick={handleBackdrop}
        >
            <div class={styles.content}>
                <Show when={!!props.onClose}>
                    <div class={styles.close} onClick={() => props.onClose?.()}>
                        <IoClose />
                    </div>
                </Show>
                {props.children}
            </div>
        </div>
    )
}

export default Modal
