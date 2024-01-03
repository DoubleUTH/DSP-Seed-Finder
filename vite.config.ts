import { defineConfig } from "vite"
import solid from "vite-plugin-solid"
import path from "path"

export default defineConfig({
    root: path.resolve(process.cwd(), "web"),
    base: "/DSP-Seed-Finder/",
    build: {
        outDir: path.resolve(process.cwd(), "dist"),
        emptyOutDir: true,
    },
    plugins: [solid()],
    resolve: {
        alias: [
            {
                find: "worldgen-wasm",
                replacement: path.resolve(process.cwd(), "pkg"),
            },
        ],
    },
})
