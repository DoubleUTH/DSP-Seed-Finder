import { defineConfig } from "@lingui/cli"
import { ALL_LANGS } from "./web/src/constants"

export default defineConfig({
    locales: ALL_LANGS as unknown as string[],
    catalogs: [
        {
            path: "<rootDir>/web/i18n/{locale}",
            include: ["<rootDir>/web/src"],
            exclude: ["**/node_modules/**"],
        },
    ],
    macro: {
        corePackage: ["#linguiCore"],
        jsxPackage: ["#lingui"],
    },
})
