import { i18n } from "@lingui/core"
import { Show, createContext, createResource, useContext } from "solid-js"
import { setLanguage, useStore } from "./store"
import type { msg, plural, select, selectOrdinal, t } from "@lingui/core/macro"
import type { JSX, ParentComponent } from "solid-js"

export type TFunc = typeof t

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
    const [t] = createResource(
        () => store.settings.language,
        async (lang) => {
            setLanguage(lang)
            const { messages } = await import(`../i18n/${lang}.po`)
            i18n.load(lang, messages)
            i18n.activate(lang)
            return i18n._.bind(i18n) as any
        },
    )

    return (
        <Show when={t()}>
            <Context.Provider value={(...args) => t()(...args)}>
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
        const { values, components } = getInterpolationValuesAndComponents(
            props as any,
        )
        const { id } = props as any
        const _translation = _(id, values!)
        return formatElements(_translation, components)
    }

    return translation()
}

export type TransProps = {
    id: string
    values?: Record<string, any>
    components?: { [key: string]: JSX.Element }
}

const getInterpolationValuesAndComponents = (data: TransProps) => {
    if (!data.values) {
        return {
            values: undefined,
            components: data.components,
        }
    }

    const values = { ...data.values }
    const components = { ...data.components }
    Object.entries(data.values).forEach(([key, valueForKey]) => {
        // simple scalars should be processed as values to be able to apply formatting
        if (
            typeof valueForKey === "string" ||
            typeof valueForKey === "number"
        ) {
            return
        }
        const index = Object.keys(components).length
        // react components, arrays, falsy values, all should be processed as JSX children
        components[index] = <>{valueForKey}</>
        values[key] = `<${index}/>`
    })
    return { values, components }
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
