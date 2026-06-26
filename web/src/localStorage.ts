import { ALL_LANGS, DEFAULT_LANG } from "./constants"

const localStorageThemeKey = "dsp-seed-finder-theme"
const localStorageLanguageKey = "dsp-seed-finder-language"
const localStorageStarSearchRulesKey = "dsp-seed-finder-star-search-rules"

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

export function getInitialStarSearchRules(): SimpleRule[][] {
    const value = localStorage.getItem(localStorageStarSearchRulesKey)
    if (!value) return []
    try {
        return JSON.parse(value)
    } catch (_) {
        return []
    }
}

export function toggleDarkMode(wasDarkMode: boolean) {
    const isDarkMode = !wasDarkMode
    localStorage.setItem(localStorageThemeKey, isDarkMode ? "dark" : "light")
    return isDarkMode
}

export function toggleLanguage(previousLanguage: Lang) {
    const language = previousLanguage === "en" ? "zh-CN" : "en"
    localStorage.setItem(localStorageLanguageKey, language)
    return language
}

export function setStarSearchRules(value: SimpleRule[][]) {
    localStorage.setItem(localStorageStarSearchRulesKey, JSON.stringify(value))
}
