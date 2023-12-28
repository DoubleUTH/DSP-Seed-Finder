import styles from "./Select.module.css"
import { For, JSX, createEffect, createSignal, onCleanup } from "solid-js"
import clsx from "clsx"
import { computePosition, flip } from "@floating-ui/dom"

function Select<T>(props: {
    class?: string
    value?: T
    onChange?: (t: T) => void
    getLabel: (t: T) => JSX.Element
    isSelected: (t: T) => boolean
    options: T[]
    onSearch?: (text: string) => void
}): JSX.Element {
    let select: HTMLDivElement
    let dropdown: HTMLDivElement

    const [focus, setFocus] = createSignal(false)

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
            class={clsx(styles.select, props.class, focus() && styles.focus)}
        >
            <div class={styles.content} onClick={() => setFocus(true)}>
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
