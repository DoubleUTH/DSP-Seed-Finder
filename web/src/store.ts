import { createContext, useContext } from "solid-js"
import { SetStoreFunction } from "solid-js/store"
import { defaultResourceMultipler, defaultStarCount } from "./util"

const localStorageKey = "dsp-seed-finder-theme";
const initializeTheme = () => {
    let theme;

    if (typeof localStorage !== "undefined" && localStorage.getItem("theme")) {
        theme = localStorage.getItem(localStorageKey) === "dark" ? "dark" : "light";
    } else if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
        theme = "dark";
    } else {
        theme = "light";
    }
    return theme;
}

export const setDarkMode = (isDarkMode: boolean) => {
    if (typeof localStorage !== "undefined") {
        localStorage.setItem(localStorageKey, isDarkMode ? "dark" : "light");
    }
}

export const defaultStore: Store = {
    settings: {
        darkMode: initializeTheme() === "dark",
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
