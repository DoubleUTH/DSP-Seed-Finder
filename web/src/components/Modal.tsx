import styles from "./Modal.module.css"
import { ParentComponent } from "solid-js"
import clsx from "clsx"
import { AiOutlineClose } from "solid-icons/ai"

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
                    <AiOutlineClose />
                </div>
                {props.children}
            </div>
        </div>
    )
}

export default Modal
