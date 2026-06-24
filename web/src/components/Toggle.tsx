import styles from "~styles"
import { Component } from "solid-js"
import clsx from "clsx"

const Toggle: Component<{
    class?: string
    value: boolean
    onChange?: (value: boolean) => void
    disabled?: boolean
}> = (props) => {
    return (
        <div
            class={clsx(
                props.class,
                styles.toggle,
                props.value && styles.active,
            )}
            onClick={() => !props.disabled && props.onChange?.(!props.value)}
        >
            <div class={styles.slider} />
        </div>
    )
}

export default Toggle
