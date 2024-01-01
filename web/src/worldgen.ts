import { WorldGenBrowser } from "./worldgen/browser"
import { WorldGenNative } from "./worldgen/native"

const browser: WorldGen = new WorldGenBrowser()
const native: WorldGen = new WorldGenNative()

export function getWorldGen(nativeMode: boolean): WorldGen {
    return nativeMode ? native : browser
}
