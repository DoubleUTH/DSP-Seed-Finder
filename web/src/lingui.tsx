import { i18n } from "@lingui/core"
import { Show, createContext, createResource, useContext } from "solid-js"
import { useStore } from "./store"
import type { msg, plural, select, selectOrdinal } from "@lingui/core/macro"
import type { JSX, ParentComponent } from "solid-js"
import { loadLanguage, TFunc } from "./linguiCore"

interface I18nContext {
    i18n: typeof i18n
    t: TFunc
    plural: typeof plural
    select: typeof select
    selectOrdinal: typeof selectOrdinal
    msg: typeof msg
}

const Context = createContext<TFunc>(undefined as unknown as TFunc)

export const I18nProvider: ParentComponent = (props) => {
    const [store] = useStore()
    const [_t] = createResource(
        () => store.settings.language,
        async (lang) => {
            await loadLanguage(lang)
            return i18n._.bind(i18n) as any
        },
    )

    return (
        <Show when={_t()}>
            <Context.Provider value={(...args) => _t()(...args)}>
                {props.children}
            </Context.Provider>
        </Show>
    )
}

export function useLingui() {
    const t = useContext(Context)
    return {
        i18n,
        _: t || (i18n._.bind(i18n) as any),
    } as unknown as I18nContext
}

export const Trans: ParentComponent = (props) => {
    const { _ } = useLingui() as any
    const translation = () => {
        const { id, values, components } = props as any
        return formatElements(_(id, values!), components)
    }

    return <>{translation()}</>
}

export type TransProps = {
    id: string
    values?: Record<string, any>
    components?: { [key: string]: JSX.Element }
}

const tagRe = /<([a-zA-Z0-9]+)\/>/

function formatElements(
    value: string,
    elements: { [key: string]: JSX.Element } = {},
): string | JSX.ArrayElement {
    const parts = value.split(tagRe)
    if (parts.length === 1) return value
    return parts.map((part, i) => (i % 2 === 1 ? elements[parts[i]!]! : part))
}
