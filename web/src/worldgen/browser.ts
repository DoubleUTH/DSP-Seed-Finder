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

export class WorldGenImpl implements WorldGen {
    private readonly workerPool = new Set<Worker>()
    concurrency: number = Math.max(navigator.hardwareConcurrency, 1)

    async generate(gameDesc: GameDesc): Promise<Galaxy> {
        const worker = new WorldgenWorker()
        this.workerPool.add(worker)
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
            this.workerPool.delete(worker)
        }
    }

    find(
        gameDesc: Omit<GameDesc, "seed">,
        range: [number, number],
        rule: Rule,
    ): AsyncGenerator<Galaxy> {
        let currentSeed = range[0]
        const finalSeed = range[1]

        const workers = Array.from({
            length: Math.min(this.concurrency, finalSeed - currentSeed + 1),
        }).map(() => {
            const worker = new WorldgenWorker()
            this.workerPool.add(worker)
            return worker
        })

        // eslint-disable-next-line @typescript-eslint/no-this-alias
        const self = this

        async function* run(worker: Worker): AsyncGenerator<Galaxy> {
            let resolveFn: ((data: Galaxy) => void) | undefined
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
                    const result = await new Promise<Galaxy>((resolve) => {
                        resolveFn = resolve
                    })
                    yield result
                    if (currentSeed > finalSeed) break
                    worker.postMessage({
                        type: FIND_NEXT_NAME,
                        input: currentSeed++,
                    })
                }
            } finally {
                worker.removeEventListener("message", eventHandler)
                worker.terminate()
                self.workerPool.delete(worker)
            }
        }

        return combineGenerators(workers.map(run))
    }

    destroy() {
        for (const worker of this.workerPool) {
            worker.terminate()
        }
        this.workerPool.clear()
    }
}
