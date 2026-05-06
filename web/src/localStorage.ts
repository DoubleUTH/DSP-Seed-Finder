import { ALL_LANGS, DEFAULT_LANG } from "./constants"

const localStorageThemeKey = "dsp-seed-finder-theme"
const localStorageLanguageKey = "dsp-seed-finder-language"

export function isInitialDarkMode(initialValue: boolean) {
    const value = localStorage.getItem(localStorageThemeKey)
    if (value != null) return value === "dark"
    return initialValue
}

export function getInitialLanguage(): Lang {
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
