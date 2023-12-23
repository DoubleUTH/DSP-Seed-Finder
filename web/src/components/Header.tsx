import styles from "./Header.module.css"
import { useStore } from "../store"
import { A } from "@solidjs/router"
import { AiOutlineSetting } from "solid-icons/ai"
import { Component } from "solid-js"

const Header: Component = () => {
    const [, setStore] = useStore()

    return (
        <div class={styles.header}>
            <div class={styles.title}>DSP Seed Finder</div>
            <div class={styles.buttons}>
                <A href="/find" class={styles.button}>
                    Star Finder
                </A>
                <A href="/galaxy" class={styles.button}>
                    Galaxy Viewer
                </A>
            </div>
            <div class={styles.icons}>
                <div
                    class={styles.icon}
                    onClick={() => setStore("modals", "settings", true)}
                >
                    <AiOutlineSetting />
                </div>
            </div>
        </div>
    )
}

export default Header
