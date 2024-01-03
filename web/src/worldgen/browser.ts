import WorldgenWorker from "./worldgen.worker?worker"

const GENERATE_NAME = "generate"
const FIND_NAME = "find"
const FIND_NEXT_NAME = "next"

export class WorldGenBrowser implements WorldGen {
    private _stop: () => void = () => {}

    async generate(gameDesc: GameDesc): Promise<Galaxy> {
        const worker = new WorldgenWorker()
        try {
            const result = await new Promise<Galaxy>((resolve) => {
                const eventHandler = (ev: MessageEvent) => {
                    const message = ev.data
                    if (message.type === GENERATE_NAME) {
                        worker.removeEventListener("message", eventHandler)
                        resolve(message.data)
                    }
                }
                worker.addEventListener("message", eventHandler)
                worker.postMessage({ type: GENERATE_NAME, input: gameDesc })
            })
            return result
        } finally {
            worker.terminate()
        }
    }

    find({
        gameDesc,
        range,
        rule,
        concurrency,
        autosave,
        onResult,
        onProgress,
        onComplete,
    }: FindOptions) {
        let currentSeed = range[0]
        const endSeed = range[1]

        let stopped = false
        this._stop = () => {
            stopped = true
        }

        const maxWorker = Math.min(concurrency, endSeed - currentSeed)
        let progressEnd = currentSeed
        const pendingSeeds = new Set<integer>()
        let done = maxWorker
        let lastNotify = Date.now()

        function run(worker: Worker) {
            const eventHandler = (ev: MessageEvent) => {
                const message = ev.data
                if (message.type === FIND_NAME) {
                    const result: FindResult = message.data
                    const seed = result.seed
                    if (result.indexes.length > 0) {
                        onResult?.(result)
                    }
                    if (progressEnd === seed) {
                        ++progressEnd
                        while (pendingSeeds.delete(progressEnd)) {
                            ++progressEnd
                        }
                    } else {
                        pendingSeeds.add(seed)
                    }
                    const now = Date.now()
                    if (now - lastNotify >= autosave * 1000) {
                        lastNotify = now
                        onProgress?.(progressEnd)
                    }
                    if (!stopped && currentSeed < endSeed) {
                        worker.postMessage({
                            type: FIND_NEXT_NAME,
                            input: currentSeed++,
                        })
                    } else {
                        worker.terminate()
                        if (--done === 0) {
                            onProgress?.(progressEnd)
                            onComplete?.()
                        }
                    }
                }
            }
            worker.addEventListener("message", eventHandler)
            worker.postMessage({
                type: FIND_NAME,
                input: { game: { ...gameDesc, seed: currentSeed++ }, rule },
            })
        }

        for (let i = 0; i < maxWorker; ++i) {
            run(new WorldgenWorker())
        }
    }

    stop() {
        this._stop()
        this._stop = () => {}
    }
}
