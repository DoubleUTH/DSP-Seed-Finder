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
                        console.log(message.data)
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
        onProgress,
        onComplete,
    }: {
        gameDesc: Omit<GameDesc, "seed">
        range: [integer, integer]
        rule: Rule
        concurrency: integer
        onProgress?: (current: number, results: FindResult[]) => void
        onComplete?: () => void
        onInterrupt?: () => void
    }) {
        let currentSeed = range[0]
        const finalSeed = range[1]

        let stopped = false
        this._stop = () => {
            stopped = true
        }

        const maxWorker = Math.min(concurrency, finalSeed - currentSeed + 1)
        let progressStart = currentSeed
        let progressEnd = currentSeed
        const pendingSeeds = new Set<integer>()
        let results: FindResult[] = []
        let done = maxWorker

        function run(worker: Worker) {
            const eventHandler = (ev: MessageEvent) => {
                const message = ev.data
                if (message.type === FIND_NAME) {
                    const result: FindResult = message.data
                    const seed = result.seed
                    if (result.indexes.length > 0) {
                        results.push(result)
                    }
                    if (progressEnd === seed) {
                        ++progressEnd
                        while (pendingSeeds.delete(progressEnd)) {
                            ++progressEnd
                        }
                        if (progressEnd >= progressStart + 1000) {
                            progressStart = progressEnd
                            onProgress?.(progressEnd, results)
                            results = []
                        }
                    } else {
                        pendingSeeds.add(seed)
                    }
                    if (!stopped && currentSeed <= finalSeed) {
                        worker.postMessage({
                            type: FIND_NEXT_NAME,
                            input: currentSeed++,
                        })
                    } else {
                        worker.terminate()
                        if (--done === 0) {
                            onProgress?.(progressEnd, results)
                            results = []
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
