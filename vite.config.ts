import { defineConfig } from "vite"
import solid from "vite-plugin-solid"
import path from "path"

export default defineConfig({
    root: path.resolve(process.cwd(), "web"),
    build: {
        outDir: path.resolve(process.cwd(), "dist"),
    },
    plugins: [solid()],
    resolve: {
        alias: [
            {
                find: "worldgen-wasm",
                replacement: path.resolve(process.cwd(), "pkg"),
            },
            {
                find: "worldgen-impl",
                replacement: path.resolve(
                    process.cwd(),
                    "web/src/worldgen",
                    "browser.ts",
                ),
            },
        ],
    },
    server: {
        proxy: {
            "/ws": {
                target: "ws://127.0.0.1:9000",
                ws: true,
            },
        },
    },
})
