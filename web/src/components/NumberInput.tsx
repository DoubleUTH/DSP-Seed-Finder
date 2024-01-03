import clsx from "clsx"
import styles from "./NumberInput.module.css"
import { Component, batch, createRenderEffect, createSignal } from "solid-js"

const NumberInput: Component<{
    class?: string
    value: number
    onChange?: (value: number) => void
    onBlur?: () => void
    error?: boolean
    emptyValue: number
    disabled?: boolean
    maxLength?: number
}> = (props) => {
    const getText = () =>
        props.value === props.emptyValue ? "" : String(props.value)
    // eslint-disable-next-line solid/reactivity
    const [text, setText] = createSignal(getText())

    function handleInput(value: string) {
        batch(() => {
            setText(value)
            if (value) {
                const num = Number(value)
                if (!Number.isNaN(num)) {
                    props.onChange?.(Number(num))
                } else {
                    props.onChange?.(props.emptyValue)
                }
            } else {
                props.onChange?.(props.emptyValue)
            }
        })
    }

    createRenderEffect(() => {
        const t = text()
        if (t === "" && props.value !== props.emptyValue) {
            setText(getText())
        } else {
            const num = Number(t)
            if (!Number.isNaN(num) && props.value !== num) {
                setText(getText())
            }
        }
    })

    return (
        <input
            class={clsx(
                styles.input,
                props.class,
                props.error && styles.error,
                props.disabled && styles.disabled,
            )}
            onBlur={() => props.onBlur?.()}
            onInput={(ev) => handleInput(ev.currentTarget.value)}
            value={text()}
            maxLength={props.maxLength}
            pattern="\\d+"
            disabled={props.disabled}
        />
    )
}

export default NumberInput
