import WorldgenWorker from "./worldgen.worker?worker"

const TYPE_GENERATE = "generate"
const TYPE_FIND = "find"
const TYPE_NEXT = "next"

interface Batch {
    id: integer
    seeds: integer[]
}

function* generateBatchFromRange(
    batchSize: integer,
    nextBatchId: integer,
    range: InternalFindOptions["range"],
): Generator<Batch, void, void> {
    let current = range[0] + batchSize * nextBatchId
    const end = range[1]
    if (current >= end) return
    do {
        const next = Math.min(current + batchSize, end)
        const seeds = Array.from({ length: next - current }).map(
            (_, i) => current + i,
        )
        yield { id: nextBatchId++, seeds }
        current = next
    } while (current < end)
}

export class WorldGenBrowser implements WorldGen {
    private stopped = false

    async generate(seed: integer, gameDesc: GameParameters): Promise<Galaxy> {
        const worker = new WorldgenWorker()
        try {
            const result = await new Promise<Galaxy>((resolve) => {
                const eventHandler = (ev: MessageEvent) => {
                    const message = ev.data
                    if (message.type === TYPE_GENERATE) {
                        worker.removeEventListener("message", eventHandler)
                        resolve(message.data)
                    }
                }
                worker.addEventListener("message", eventHandler)
                worker.postMessage({
                    type: TYPE_GENERATE,
                    input: { seed, gameDesc },
                })
            })
            return result
        } finally {
            worker.terminate()
        }
    }

    async find({
        batchSize,
        nextBatchId,
        gameDesc,
        range,
        rule,
        concurrency,
        onBatchResult,
    }: InternalFindOptions) {
        this.stopped = false
        const batch = generateBatchFromRange(batchSize, nextBatchId, range)

        const run = (worker: Worker) => {
            let currentBatch = batch.next()
            if (currentBatch.done) {
                worker.terminate()
                return
            }
            let resolved = () => {}
            const promise = new Promise<void>((resolve) => {
                resolved = resolve
            })
            worker.addEventListener("message", (ev) => {
                const message = ev.data
                if (message.type === TYPE_FIND) {
                    const result: integer[] = message.data
                    onBatchResult(currentBatch.value!.id, result)
                    if (this.stopped) {
                        worker.terminate()
                        resolved()
                    } else {
                        currentBatch = batch.next()
                        if (currentBatch.done) {
                            worker.terminate()
                            resolved()
                        } else {
                            worker.postMessage({
                                type: TYPE_NEXT,
                                input: currentBatch.value.seeds,
                            })
                        }
                    }
                }
            })
            worker.postMessage({
                type: TYPE_FIND,
                input: {
                    game: gameDesc,
                    rule,
                    seeds: currentBatch.value.seeds,
                },
            })
            return promise
        }

        await Promise.all(
            Array.from({ length: concurrency }).map(() =>
                run(new WorldgenWorker()),
            ),
        )
    }

    stop() {
        this.stopped = true
    }
}
