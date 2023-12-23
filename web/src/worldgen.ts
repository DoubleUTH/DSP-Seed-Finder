import { useStore } from "./store"
import { WorldGenBrowser } from "./worldgen/browser"
import { WorldGenNative } from "./worldgen/native"

const browser: WorldGen = new WorldGenBrowser()
const native: WorldGen = new WorldGenNative()
const mixed: WorldGen = {
    generate: (gameDesc) => browser.generate(gameDesc),
    find: (options) => native.find(options),
    stop: () => native.stop(),
}

export function useWorldGen(): () => WorldGen {
    const [store] = useStore()

    return () => (store.settings.nativeMode ? mixed : browser)
}
