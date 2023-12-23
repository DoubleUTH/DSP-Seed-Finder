import styles from "./Button.module.css"
import { ParentComponent } from "solid-js"
import clsx from "clsx"

const Button: ParentComponent<{
    class?: string
    type?: "button" | "submit" | "reset"
    kind?: "solid" | "outline"
    disabled?: boolean
    onClick?: (ev: MouseEvent) => void
}> = (props) => {
    return (
        <button
            type={props.type || "button"}
            class={clsx(
                styles.button,
                props.class,
                props.kind === "outline" ? styles.outline : styles.solid,
            )}
            disabled={props.disabled}
            onClick={(ev) => props.onClick?.(ev)}
        >
            {props.children}
        </button>
    )
}

export default Button
