import { WorldGenBrowser } from "./worldgen/browser"
import { WorldGenNative } from "./worldgen/native"

const browser: WorldGen = new WorldGenBrowser()
const native: WorldGen = new WorldGenNative()

const ALWAYS_NATIVE = false

export function getWorldGen(nativeMode: boolean): WorldGen {
    return ALWAYS_NATIVE ? native : nativeMode ? native : browser
}
