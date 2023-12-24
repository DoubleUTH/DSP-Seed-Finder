import { createContext, useContext } from "solid-js"
import { SetStoreFunction } from "solid-js/store"

export const defaultStore: Store = {
    settings: {
        darkMode: window.matchMedia("(prefers-color-scheme: dark)").matches,
        concurrency: Math.max(navigator.hardwareConcurrency, 1),
        nativeMode: true,
        starCount: 64,
        resourceMultiplier: 1,
    },
    modals: {
        settings: false,
    },
    galaxys: {},
}

type ContextType = [get: Store, set: SetStoreFunction<Store>]

export const StoreContext = createContext<ContextType>(
    undefined as unknown as ContextType,
)

export function useStore() {
    return useContext(StoreContext)
}
