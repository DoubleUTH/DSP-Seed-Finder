import { defineConfig } from "vite"
import solid, { Options } from "vite-plugin-solid"
import { lingui } from "@lingui/vite-plugin"
import path from "node:path"

const babelConfig: NonNullable<Options["babel"]> = {
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
}

export default defineConfig({
    root: path.resolve(process.cwd(), "web"),
    base: "/DSP-Seed-Finder/",
    build: {
        outDir: path.resolve(process.cwd(), "dist"),
        emptyOutDir: true,
    },
    worker: {
        format: "es",
        plugins: () => [
            solid({
                babel: babelConfig,
            }),
            lingui(),
        ],
    },
    plugins: [
        solid({
            babel: babelConfig,
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
                find: "#linguiCore",
                replacement: path.resolve(
                    process.cwd(),
                    "web",
                    "src",
                    "linguiCore.ts",
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
