import { ParentComponent, createEffect, createSignal } from "solid-js"
import clsx from "clsx"
import styles from "./Tooltip.module.css"
import { computePosition, flip } from "@floating-ui/dom"

const Tooltip: ParentComponent<{ text: string; class?: string }> = (props) => {
    let root: HTMLDivElement
    let popup: HTMLDivElement

    const [focus, setFocus] = createSignal(false)

    createEffect(() => {
        if (focus()) {
            popup!.style.display = ""
            computePosition(root!, popup!, {
                strategy: "fixed",
                placement: "top",
                middleware: [flip({ fallbackPlacements: ["bottom"] })],
                // eslint-disable-next-line solid/reactivity
            }).then(({ x, y }) => {
                popup!.style.left = x + "px"
                popup!.style.top = y + "px"
            })
        } else {
            popup!.style.display = "none"
        }
    })

    return (
        <span
            ref={root!}
            class={clsx(styles.tooltip, props.class)}
            onMouseEnter={() => setFocus(true)}
            onMouseLeave={() => setFocus(false)}
        >
            {props.children}
            <div ref={popup!} class={styles.popup}>
                {props.text}
            </div>
        </span>
    )
}

export default Tooltip
