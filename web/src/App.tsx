import { createStore } from "solid-js/store"
import clsx from "clsx"
import styles from "./App.module.css"
import { StoreContext, defaultStore } from "./store"
import Header from "./components/Header"
import { ParentComponent } from "solid-js"
import Settings from "./components/Settings"

const App: ParentComponent = (props) => {
    const [store, setStore] = createStore<Store>(defaultStore)

    return (
        <StoreContext.Provider value={[store, setStore]}>
            <div
                class={clsx(
                    styles.app,
                    store.settings.darkMode ? styles.dark : styles.light,
                )}
            >
                <Header />
                {props.children}
                <Settings />
            </div>
        </StoreContext.Provider>
    )
}

export default App
