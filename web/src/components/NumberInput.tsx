import clsx from "clsx"
import styles from "./NumberInput.module.css"
import { Component } from "solid-js"

const NumberInput: Component<{
    class?: string
    value: number
    onChange?: (value: number) => void
    min?: number
    max?: number
    step?: number
}> = (props) => {
    return (
        <input
            class={clsx(styles.input, props.class)}
            type="number"
            onInput={(ev) => props.onChange?.(Number(ev.currentTarget.value))}
            value={props.value}
            min={props.min}
            max={props.max}
            step={props.step}
        />
    )
}

export default NumberInput
