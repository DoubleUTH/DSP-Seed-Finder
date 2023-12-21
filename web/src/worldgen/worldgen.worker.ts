import { TinyEmitter } from "tiny-emitter"
import init, { generate, findStars } from "worldgen-wasm"

const GENERATE_NAME = "generate"
const FIND_NAME = "find"
const FIND_NEXT_NAME = "next"

const initPromise = init()
const emitter = new TinyEmitter()

const worldgen = {
    async found(result: any) {
        const wait = new Promise<number | null>((resolve) =>
            emitter.once(FIND_NEXT_NAME, resolve),
        )
        self.postMessage({ type: FIND_NAME, data: result })
        const nextSeed = await wait
        return nextSeed
    },
}

;(self as any).worldgen = worldgen

self.onmessage = (ev) => {
    const { type, input } = ev.data

    if (type === GENERATE_NAME) {
        const { seed, resourceMultiplier = 1, starCount = 64 } = input

        initPromise.then(() => {
            const result = generate({ seed, starCount, resourceMultiplier })
            self.postMessage({ type: GENERATE_NAME, data: result })
        })
    } else if (type === FIND_NAME) {
        const {
            game: { seed = 0, resourceMultiplier = 1, starCount = 64 },
            rule,
        } = input

        initPromise.then(() => {
            findStars({ seed, starCount, resourceMultiplier }, rule)
        })
    } else if (type === FIND_NEXT_NAME) {
        emitter.emit(FIND_NEXT_NAME, input)
    }
}
