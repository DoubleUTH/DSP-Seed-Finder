import styles from "./Header.module.css"
import { useStore } from "../store"
import { A } from "@solidjs/router"
import { IoContrast, IoLogoGithub } from "solid-icons/io"
import { Component } from "solid-js"

const Header: Component = () => {
    const [, setStore] = useStore()

    return (
        <div class={styles.header}>
            <div class={styles.title}>DSP Seed Finder</div>
            <div class={styles.buttons}>
                <A href="/" class={styles.button}>
                    Star Finder
                </A>
                <A href="/galaxy" class={styles.button}>
                    Galaxy Viewer
                </A>
            </div>
            <div class={styles.icons}>
                <a
                    href="https://github.com/DoubleUTH/DSP-Seed-Finder"
                    target="_blank"
                    class={styles.icon}
                    onClick={() => setStore("settings", "darkMode", (x) => !x)}
                >
                    <IoLogoGithub />
                </a>
                <div
                    class={styles.icon}
                    onClick={() => setStore("settings", "darkMode", (x) => !x)}
                >
                    <IoContrast />
                </div>
            </div>
        </div>
    )
}

export default Header
