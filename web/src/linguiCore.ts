import { i18n } from "@lingui/core"
import type { t as _t } from "@lingui/core/macro"

export type TFunc = typeof _t
export const t: TFunc = i18n._.bind(i18n) as any

export async function loadLanguage(lang: Lang) {
    const { messages } = await import(`../i18n/${lang}.po`)
    i18n.load(lang, messages)
    i18n.activate(lang)
    return lang
}
