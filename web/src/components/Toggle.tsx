import styles from "./Toggle.module.css"
import { Component } from "solid-js"
import clsx from "clsx"

const Toggle: Component<{
    value: boolean
    onChange?: (value: boolean) => void
    disabled?: boolean
}> = (props) => {
    return (
        <div
            class={clsx(styles.toggle, props.value && styles.active)}
            onClick={() => !props.disabled && props.onChange?.(!props.value)}
        >
            <div class={styles.slider} />
        </div>
    )
}

export default Toggle
