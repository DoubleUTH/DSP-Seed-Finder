import { createContext, useContext } from "solid-js"
import { SetStoreFunction } from "solid-js/store"
import { getDefaultParams } from "./util"
import { isInitialDarkMode, getInitialLanguage } from "./localStorage"

export const defaultStore: Store = {
    settings: {
        darkMode: isInitialDarkMode(
            window.matchMedia("(prefers-color-scheme: dark)").matches,
        ),
        language: getInitialLanguage(),
        view: getDefaultParams(),
    },
    searching: false,
}

type ContextType = [get: Store, set: SetStoreFunction<Store>]

export const StoreContext = createContext<ContextType>(
    undefined as unknown as ContextType,
)

export function useStore() {
    return useContext(StoreContext)
}
