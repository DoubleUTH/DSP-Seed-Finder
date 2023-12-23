import WorldgenWorker from "./worldgen.worker?worker"

const GENERATE_NAME = "generate"
const FIND_NAME = "find"
const FIND_NEXT_NAME = "next"

async function* combineGenerators<T>(iterable: AsyncGenerator<T>[]) {
    const asyncIterators = Array.from(iterable, (o) =>
        o[Symbol.asyncIterator](),
    )
    let count = asyncIterators.length
    const never = new Promise<any>(() => {})
    async function getNext(asyncIterator: AsyncIterator<T>, index: number) {
        const result = await asyncIterator.next()
        return { index, result }
    }
    const nextPromises = asyncIterators.map(getNext)
    while (count) {
        const { index, result } = await Promise.race(nextPromises)
        if (result.done) {
            nextPromises[index] = never
            count--
        } else {
            nextPromises[index] = getNext(asyncIterators[index]!, index)
            yield result.value
        }
    }
}

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
    }: {
        gameDesc: Omit<GameDesc, "seed">
        range: [integer, integer]
        rule: Rule
        concurrency: integer
    }): AsyncGenerator<Galaxy> {
        let currentSeed = range[0]
        const finalSeed = range[1]

        const workers = Array.from({
            length: Math.min(concurrency, finalSeed - currentSeed + 1),
        }).map(() => new WorldgenWorker())

        let stopped = false
        this._stop = () => {
            stopped = true
        }

        async function* run(worker: Worker): AsyncGenerator<Galaxy> {
            let resolveFn: ((data?: Galaxy) => void) | undefined
            const eventHandler = (ev: MessageEvent) => {
                const message = ev.data
                if (message.type === FIND_NAME) {
                    resolveFn?.(message.data)
                }
            }
            worker.addEventListener("message", eventHandler)
            try {
                worker.postMessage({
                    type: FIND_NAME,
                    input: { game: { ...gameDesc, seed: currentSeed++ }, rule },
                })

                for (;;) {
                    const result = await new Promise<Galaxy | undefined>(
                        (resolve) => {
                            resolveFn = resolve
                        },
                    )
                    if (!result) break
                    yield result
                    if (currentSeed > finalSeed) break
                    worker.postMessage({
                        type: FIND_NEXT_NAME,
                        input: currentSeed++,
                    })
                    if (stopped) break
                }
            } finally {
                worker.removeEventListener("message", eventHandler)
                worker.terminate()
            }
        }

        return combineGenerators(workers.map(run))
    }

    stop() {
        this._stop()
        this._stop = () => {}
    }
}
