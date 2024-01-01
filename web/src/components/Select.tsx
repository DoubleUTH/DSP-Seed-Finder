import styles from "./Select.module.css"
import { For, JSX, createEffect, createSignal, onCleanup } from "solid-js"
import clsx from "clsx"
import { computePosition, flip } from "@floating-ui/dom"

function Select<T>(props: {
    class?: string
    value?: T
    placeholder?: string
    onChange?: (t: T) => void
    getLabel: (t: T) => JSX.Element
    options: T[]
    isSelected?: (t: T) => boolean
    error?: boolean
    disabled?: boolean
}): JSX.Element {
    let select: HTMLDivElement
    let dropdown: HTMLDivElement

    const [focus, setFocus] = createSignal(false)

    function isSelected(option: T): boolean {
        return props.isSelected?.(option) || props.value === option
    }

    createEffect(() => {
        const handler = (ev: MouseEvent) => {
            if (!select!.contains(ev.target as Node)) {
                setFocus(false)
            }
        }
        document.body.addEventListener("click", handler)
        onCleanup(() => document.body.removeEventListener("click", handler))
    })

    createEffect(() => {
        if (focus()) {
            dropdown!.style.display = ""
            dropdown.style.width = select.clientWidth + "px"
            computePosition(select!, dropdown!, {
                strategy: "fixed",
                placement: "bottom",
                middleware: [flip({ fallbackPlacements: ["top"] })],
                // eslint-disable-next-line solid/reactivity
            }).then(({ x, y }) => {
                dropdown!.style.left = x + "px"
                dropdown!.style.top = y + "px"
                const selected = props.options.findIndex(isSelected)
                if (selected > -1) {
                    dropdown.children[selected]!.scrollIntoView({
                        behavior: "instant",
                        block: "center",
                        inline: "start",
                    })
                }
            })
        } else {
            dropdown!.style.display = "none"
        }
    })

    function onClick() {
        if (!props.disabled) {
            setFocus(true)
        }
    }

    return (
        <div
            ref={select!}
            class={clsx(
                styles.select,
                props.class,
                focus() && styles.focus,
                props.error && styles.error,
                props.disabled && styles.disabled,
            )}
        >
            <div class={styles.content} onClick={onClick}>
                {props.value !== undefined
                    ? props.getLabel(props.value)
                    : props.placeholder}
            </div>
            <div ref={dropdown!} class={styles.dropdown}>
                <For each={props.options}>
                    {(option) => (
                        <div
                            class={clsx(
                                styles.item,
                                isSelected(option) && styles.selected,
                            )}
                            onClick={() => {
                                setFocus(false)
                                props.onChange?.(option)
                            }}
                        >
                            {props.getLabel(option)}
                        </div>
                    )}
                </For>
            </div>
        </div>
    )
}

export default Select
