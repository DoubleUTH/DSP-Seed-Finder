import styles from "./Select.module.css"
import { For, JSX, createEffect, onCleanup } from "solid-js"
import clsx from "clsx"
import { computePosition, flip } from "@floating-ui/dom"

function Select<T>(props: {
    class?: string
    value?: T
    onChange?: (t: T) => void
    getLabel: (t: T) => JSX.Element
    isSelected: (t: T) => boolean
    options: T[]
    focus?: boolean
    onFocusChange?: (focus: boolean) => void
}): JSX.Element {
    let select: HTMLDivElement
    let dropdown: HTMLDivElement

    createEffect(() => {
        const handler = (ev: MouseEvent) => {
            if (!select!.contains(ev.target as Node)) {
                props.onFocusChange?.(false)
            }
        }
        document.body.addEventListener("click", handler)
        onCleanup(() => document.body.removeEventListener("click", handler))
    })

    createEffect(() => {
        if (props.focus) {
            dropdown!.style.display = ""
            dropdown.style.width = select.clientWidth + "px"
            computePosition(select!, dropdown!, {
                strategy: "fixed",
                placement: "bottom",
                middleware: [flip({ fallbackPlacements: ["top"] })],
            }).then(({ x, y }) => {
                dropdown!.style.left = x + "px"
                dropdown!.style.top = y + "px"
                const selected = props.options.findIndex(props.isSelected)
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

    return (
        <div
            ref={select!}
            class={clsx(
                styles.select,
                props.class,
                props.focus && styles.focus,
            )}
        >
            <div
                class={styles.content}
                onClick={() => props.onFocusChange?.(true)}
            >
                {props.value ? props.getLabel(props.value) : ""}
            </div>
            <div ref={dropdown!} class={styles.dropdown}>
                <For each={props.options}>
                    {(option) => (
                        <div
                            class={clsx(
                                styles.item,
                                props.isSelected(option) && styles.selected,
                            )}
                            onClick={() => {
                                props.onFocusChange?.(false)
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
