import init, { generate } from "worldgen-wasm"

const GENERATE_NAME = "generate"

const initPromise = init()

const id = Math.random()

self.onmessage = (ev) => {
    const { type, input } = ev.data

    if (type === GENERATE_NAME) {
        const { seed, resourceMultiplier = 1, starCount = 64 } = input

        console.log("Generating for", seed, id)

        initPromise.then(() => {
            const result = generate({ seed, starCount, resourceMultiplier })
            self.postMessage({ type: GENERATE_NAME, data: result })
        })
    }
}
