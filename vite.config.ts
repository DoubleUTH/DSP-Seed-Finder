import { defineConfig } from "vite"
import solid from "vite-plugin-solid"
import { lingui } from "@lingui/vite-plugin"
import path from "node:path"

export default defineConfig({
    root: path.resolve(process.cwd(), "web"),
    base: "/DSP-Seed-Finder/",
    build: {
        outDir: path.resolve(process.cwd(), "dist"),
        emptyOutDir: true,
        minify: false,
    },
    worker: {
        format: "es",
    },
    plugins: [
        solid({
            babel: {
                plugins: ["@lingui/babel-plugin-lingui-macro"],
                targets: ">0.5%, not dead",
                assumptions: {
                    constantReexports: true,
                    constantSuper: true,
                    enumerableModuleMeta: true,
                    ignoreFunctionLength: true,
                    ignoreToPrimitiveHint: true,
                    iterableIsArray: true,
                    mutableTemplateObject: true,
                    noClassCalls: true,
                    noDocumentAll: true,
                    noIncompleteNsImportDetection: true,
                    noNewArrows: true,
                    noUninitializedPrivateFieldAccess: true,
                    objectRestNoSymbols: true,
                    privateFieldsAsProperties: true,
                    privateFieldsAsSymbols: true,
                    pureGetters: true,
                    setClassMethods: true,
                    setComputedProperties: true,
                    setPublicClassFields: true,
                    setSpreadProperties: true,
                    skipForOfIteratorClosing: true,
                    superIsCallableConstructor: true,
                },
            },
        }),
        lingui(),
    ],
    resolve: {
        alias: [
            {
                find: "worldgen-wasm",
                replacement: path.resolve(process.cwd(), "pkg"),
            },
            {
                find: "#lingui",
                replacement: path.resolve(
                    process.cwd(),
                    "web",
                    "src",
                    "lingui.tsx",
                ),
            },
            {
                find: "@lingui/react",
                replacement: path.resolve(
                    process.cwd(),
                    "web",
                    "src",
                    "lingui.tsx",
                ),
            },
        ],
    },
})
