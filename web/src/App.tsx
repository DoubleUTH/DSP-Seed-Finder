import { createStore } from "solid-js/store"
import clsx from "clsx"
import styles from "~styles"
import { StoreContext, defaultStore } from "./store"
import Header from "./partials/Header"
import { ParentComponent, createEffect, onCleanup } from "solid-js"
import { I18nProvider } from "./lingui"

const App: ParentComponent = (props) => {
    const [store, setStore] = createStore<Store>(defaultStore)

    createEffect(() => {
        if (store.searching) {
            const unload = (ev: Event) => {
                ev.preventDefault()
            }
            window.addEventListener("beforeunload", unload)
            onCleanup(() => window.removeEventListener("beforeunload", unload))
        }
    })

    return (
        <StoreContext.Provider value={[store, setStore]}>
            <I18nProvider>
                <div
                    class={clsx(
                        styles.app,
                        store.settings.darkMode ? styles.dark : styles.light,
                    )}
                >
                    <Header />
                    <div class={styles.content}>{props.children}</div>
                    <div id="portal" />
                </div>
            </I18nProvider>
        </StoreContext.Provider>
    )
}

export default App
