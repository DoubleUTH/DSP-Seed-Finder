import styles from "./Header.module.css"
import { useStore } from "../store"
import { A } from "@solidjs/router"
import { IoContrast, IoLogoGithub } from "solid-icons/io"
import { Component } from "solid-js"
import clsx from "clsx"
import { useLingui } from "#lingui"
import { toggleDarkMode, toggleLanguage } from "../localStorage"

const Header: Component = () => {
    const [store, setStore] = useStore()
    const { t } = useLingui()

    return (
        <div class={styles.header}>
            <div class={styles.title}>{t`DSP Seed Finder`}</div>
            <div
                class={clsx(
                    styles.buttons,
                    store.searching && styles.buttonsDisabled,
                )}
            >
                <A href="/find-star" class={styles.button}>
                    {t`Star Finder`}
                </A>
                <A href="/find-galaxy" class={styles.button}>
                    {t`Galaxy Finder`}
                </A>
                <A href="/galaxy" class={styles.button}>
                    {t`Galaxy Viewer`}
                </A>
            </div>
            <div class={styles.icons}>
                <div
                    class={styles.language}
                    onClick={() => {
                        setStore("settings", "language", toggleLanguage)
                    }}
                >
                    {store.settings.language === "en" ? "中" : "En"}
                </div>
                <a
                    href="https://github.com/DoubleUTH/DSP-Seed-Finder"
                    target="_blank"
                    class={styles.icon}
                >
                    <IoLogoGithub />
                </a>
                <div
                    class={styles.icon}
                    onClick={() => {
                        setStore("settings", "darkMode", toggleDarkMode)
                    }}
                >
                    <IoContrast />
                </div>
            </div>
        </div>
    )
}

export default Header
