import { createContext, useContext } from "solid-js"
import { SetStoreFunction } from "solid-js/store"
import { defaultResourceMultipler, defaultStarCount } from "./util"

export const defaultStore: Store = {
    settings: {
        darkMode: window.matchMedia("(prefers-color-scheme: dark)").matches,
        view: {
            starCount: defaultStarCount,
            resourceMultipler: defaultResourceMultipler,
        },
    },
}

type ContextType = [get: Store, set: SetStoreFunction<Store>]

export const StoreContext = createContext<ContextType>(
    undefined as unknown as ContextType,
)

export function useStore() {
    return useContext(StoreContext)
}
