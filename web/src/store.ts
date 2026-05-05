import { createContext, useContext } from "solid-js"
import { SetStoreFunction } from "solid-js/store"
import {
    defaultHiveInitialColonize,
    defaultHiveMaxDensity,
    defaultResourceMultipler,
    defaultStarCount,
} from "./util"
import { ALL_LANGS, DEFAULT_LANG } from "./constants"

const localStorageThemeKey = "dsp-seed-finder-theme"
const localStorageLanguageKey = "dsp-seed-finder-language"

function isInitialDarkMode() {
    const value = localStorage.getItem(localStorageThemeKey)
    if (value != null) return value === "dark"
    return window.matchMedia("(prefers-color-scheme: dark)").matches
}

function getInitialLanguage(): Lang {
    const value = localStorage.getItem(localStorageLanguageKey)
    if (value != null && ALL_LANGS.includes(value as Lang)) return value as Lang
    return DEFAULT_LANG
}

export function toggleDarkMode(wasDarkMode: boolean) {
    const isDarkMode = !wasDarkMode
    localStorage.setItem(localStorageThemeKey, isDarkMode ? "dark" : "light")
    return isDarkMode
}

export function setLanguage(language: Lang) {
    localStorage.setItem(localStorageLanguageKey, language)
}

export const defaultStore: Store = {
    settings: {
        darkMode: isInitialDarkMode(),
        language: getInitialLanguage(),
        view: {
            starCount: defaultStarCount,
            resourceMultipler: defaultResourceMultipler,
            hiveInitialColonize: defaultHiveInitialColonize,
            hiveMaxDensity: defaultHiveMaxDensity,
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
