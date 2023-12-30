import styles from "./Modal.module.css"
import { ParentComponent } from "solid-js"
import clsx from "clsx"
import { IoClose } from "solid-icons/io"

const Modal: ParentComponent<{
    class?: string
    visible: boolean
    onClose?: () => void
}> = (props) => {
    function handleBackdrop(ev: MouseEvent) {
        if (ev.currentTarget === ev.target) {
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
                <div class={styles.close} onClick={() => props.onClose?.()}>
                    <IoClose />
                </div>
                {props.children}
            </div>
        </div>
    )
}

export default Modal
