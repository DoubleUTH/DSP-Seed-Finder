import { TinyEmitter } from "tiny-emitter"
import init, { generate, findStars } from "worldgen-wasm"

const TYPE_GENERATE = "generate"
const TYPE_FIND = "find"
const TYPE_NEXT = "next"

const initPromise = init()
const emitter = new TinyEmitter()

const worldgen = {
    async found(result: any) {
        const wait = new Promise<number[] | null>((resolve) =>
            emitter.once(TYPE_NEXT, resolve),
        )
        self.postMessage({ type: TYPE_FIND, data: result })
        const nextSeeds = await wait
        return nextSeeds
    },
}

;(self as any).worldgen = worldgen

self.onmessage = (ev) => {
    const { type, input } = ev.data

    if (type === TYPE_GENERATE) {
        const {
            seed,
            gameDesc: {
                resourceMultiplier = 1,
                starCount = 64,
                hiveInitialColonize = 1,
                hiveMaxDensity = 1,
                useActualVeins = true,
            },
        } = input

        initPromise.then(() => {
            const result = generate(seed, {
                starCount,
                resourceMultiplier,
                hiveInitialColonize,
                hiveMaxDensity,
                useActualVeins,
            })
            self.postMessage({ type: TYPE_GENERATE, data: result })
        })
    } else if (type === TYPE_FIND) {
        const {
            game: {
                resourceMultiplier = 1,
                starCount = 64,
                hiveInitialColonize = 1,
                hiveMaxDensity = 1,
                useActualVeins = true,
            },
            rule,
            seeds,
        } = input

        initPromise.then(() => {
            findStars(
                {
                    starCount,
                    resourceMultiplier,
                    hiveInitialColonize,
                    hiveMaxDensity,
                    useActualVeins,
                },
                rule,
                seeds,
            )
        })
    } else if (type === TYPE_NEXT) {
        emitter.emit(TYPE_NEXT, input)
    }
}
