import { createContext, useContext } from "solid-js"
import { SetStoreFunction } from "solid-js/store"
import { defaultResourceMultipler, defaultStarCount } from "./util"

const localStorageKey = "dsp-seed-finder-theme"

function isInitialDarkMode() {
    const value = localStorage.getItem(localStorageKey)
    if (value != null) return value === "dark"
    return window.matchMedia("(prefers-color-scheme: dark)").matches
}

export function toggleDarkMode(wasDarkMode: boolean) {
    const isDarkMode = !wasDarkMode
    localStorage.setItem(localStorageKey, isDarkMode ? "dark" : "light")
    return isDarkMode
}

export const defaultStore: Store = {
    settings: {
        darkMode: isInitialDarkMode(),
        view: {
            starCount: defaultStarCount,
            resourceMultipler: defaultResourceMultipler,
        },
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
