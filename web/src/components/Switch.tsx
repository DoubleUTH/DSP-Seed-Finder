import styles from "./Switch.module.css"
import { Component } from "solid-js"
import clsx from "clsx"

const Switch: Component<{
    value: boolean
    onChange?: (value: boolean) => void
}> = (props) => {
    return (
        <div
            class={clsx(styles.switch, props.value && styles.active)}
            onClick={() => props.onChange?.(!props.value)}
        >
            <div class={styles.slider} />
        </div>
    )
}

export default Switch
