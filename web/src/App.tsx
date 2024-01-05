import { createStore } from "solid-js/store"
import clsx from "clsx"
import styles from "./App.module.css"
import { StoreContext, defaultStore } from "./store"
import Header from "./partials/Header"
import { ParentComponent, createEffect, onCleanup } from "solid-js"

const App: ParentComponent = (props) => {
    const [store, setStore] = createStore<Store>(defaultStore)

    createEffect(() => {
        console.log(store.searching)
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
            <div
                class={clsx(
                    styles.app,
                    store.settings.darkMode ? styles.dark : styles.light,
                )}
            >
                <Header />
                <div class={styles.content}>{props.children}</div>
            </div>
        </StoreContext.Provider>
    )
}

export default App
