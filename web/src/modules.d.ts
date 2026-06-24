declare module "~styles" {
    const styles: CSSModuleClasses
    export default styles
}

declare module "*.po" {
    import type { Messages } from "@lingui/core"
    export const messages: Messages
}
