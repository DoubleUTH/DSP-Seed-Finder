type BatchGenerator = Generator<Int32Array<ArrayBuffer>, void, void>

function* generateBatchFromRange(
    batchSize: integer,
    nextBatchId: integer,
    range: InternalFindOptions["range"],
): BatchGenerator {
    if (Array.isArray(range)) {
        let current = range[0] + batchSize * nextBatchId
        const end = range[1]
        while (current < end) {
            const next = Math.min(current + batchSize, end)
            const array = new Int32Array(next - current + 1)
            array[0] = nextBatchId++
            for (let i = current; i < next; ++i) {
                array[i - current + 1] = i
            }
            yield array
            current = next
        }
    } else {
        let current = batchSize * nextBatchId
        const end = range.length
        while (current < end) {
            const next = Math.min(current + batchSize, end)
            const array = new Int32Array(next - current + 1)
            array[0] = nextBatchId++
            array.set(range.slice(current, next), 1)
            yield array
            current = next
        }
    }
}

function waitConnect(ws: WebSocket) {
    return new Promise<void>((resolve, reject) => {
        ws.addEventListener("error", reject)
        if (ws.readyState === WebSocket.CONNECTING) {
            const handle = () => {
                resolve()
                ws.removeEventListener("open", handle)
            }
            ws.addEventListener("open", handle)
        }
    })
}

async function connect() {
    const ws = new WebSocket("ws://127.0.0.1:62879")
    ws.binaryType = "arraybuffer"
    console.debug("connecting")
    await waitConnect(ws)
    console.debug("connected")
    return ws
}

function createSender(
    ws: WebSocket,
    generator: BatchGenerator,
    stopped: () => boolean,
): (batchId: number) => boolean {
    if (stopped()) return () => false
    const batch = generator.next()
    if (batch.done) return () => false
    ws.send(batch.value)
    let prevBatchId = batch.value[0]
    let done = false
    return (batchId) => {
        if (done || stopped()) return false
        if (prevBatchId !== batchId) return true
        const batch = generator.next()
        if (batch.done) {
            done = true
            return false
        }
        ws.send(batch.value)
        prevBatchId = batch.value[0]
        return true
    }
}

async function send(ws: WebSocket, msg: any) {
    const promise = new Promise<any>((resolve) => {
        const listener = (ev: MessageEvent) => {
            if (typeof ev.data === "string") {
                const resp = JSON.parse(ev.data)
                if (resp.type === msg.type) {
                    ws.removeEventListener("message", listener)
                    resolve(JSON.parse(ev.data))
                }
            }
        }
        ws.addEventListener("message", listener)
    })
    ws.send(JSON.stringify(msg))
    return promise
}

export class WorldGenNative implements WorldGen {
    private stopped: boolean = false

    async generate(seed: integer, gameDesc: GameParameters): Promise<Galaxy> {
        const ws = await connect()
        const resp = await send(ws, {
            type: "Generate",
            seed: seed,
            game: gameDesc,
        })
        return resp.galaxy
    }

    async find({
        batchSize,
        nextBatchId,
        range,
        gameDesc,
        concurrency,
        rule,
        onBatchResult,
        onInterrupt,
    }: InternalFindOptions) {
        this.stopped = false
        const batch = generateBatchFromRange(batchSize, nextBatchId, range)
        const ws = await connect()
        try {
            let running = true
            ws.addEventListener("close", () => {
                running = false
                onInterrupt()
            })
            const setupRes = await send(ws, {
                type: "Setup",
                concurrency,
                game: gameDesc,
                rule,
            })
            if (!setupRes?.success) return
            const stopped = () => !running || this.stopped
            let senders = [
                createSender(ws, batch, stopped),
                createSender(ws, batch, stopped),
                createSender(ws, batch, stopped),
                createSender(ws, batch, stopped),
            ].filter((sender) => sender(NaN))

            if (senders.length === 0) return

            await new Promise<void>((resolve) => {
                ws.addEventListener("message", (ev) => {
                    if (ev.data instanceof ArrayBuffer) {
                        const array = new Int32Array(ev.data)
                        const seeds = Array.from(array.slice(1))
                        const batchId = array[0]!
                        onBatchResult(batchId, seeds)
                        senders = senders.filter((sender) => sender(batchId))
                        if (senders.length === 0) {
                            resolve()
                        }
                    }
                })
            })
        } finally {
            if (ws.readyState === WebSocket.OPEN) {
                ws.close()
            }
        }
    }

    async createDatabase({
        name,
        range,
        params,
        concurrency,
        onProgress,
        onInterrupt,
    }: InternalGenerateDatabaseOptions) {
        const ws = await connect()
        try {
            ws.addEventListener("close", () => {
                onInterrupt()
            })
            ws.send(
                JSON.stringify({
                    type: "Database",
                    name,
                    concurrency,
                    range,
                    game: params,
                }),
            )

            await new Promise<void>((resolve) => {
                ws.addEventListener("message", (ev) => {
                    if (typeof ev.data === "string") {
                        const data = JSON.parse(ev.data)
                        if (data.type === "Database") {
                            onProgress(data.progress)
                        }
                        if (data.progress === range[1]) {
                            resolve()
                        }
                    }
                })
            })
        } finally {
            if (ws.readyState === WebSocket.OPEN) {
                ws.close()
            }
        }
    }

    stop() {
        this.stopped = true
    }
}
