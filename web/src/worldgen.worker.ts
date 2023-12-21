import init, { generate } from "worldgen"

const initPromise = init()

self.onmessage = (ev) => {
    const { seed, resourceMultiplier = 1, starCount = 64 } = ev.data

    initPromise.then(() => {
        const result = generate(seed, starCount, resourceMultiplier)
        self.postMessage(result)
    })
}
