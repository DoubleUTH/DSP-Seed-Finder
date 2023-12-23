function waitConnect(ws: WebSocket) {
    return new Promise<void>((resolve) => {
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
    console.log("connecting")
    await waitConnect(ws)
    console.log("connected")
    return ws
}

export class WorldGenNative implements WorldGen {
    private _stop: () => void = () => {}

    async generate(gameDesc: GameDesc): Promise<Galaxy> {
        const ws = await connect()
        const promise = new Promise<Galaxy>((resolve) => {
            ws.addEventListener("message", (ev) => {
                resolve(JSON.parse(ev.data))
                ws.close()
            })
        })
        ws.send(JSON.stringify({ type: "Generate", game: gameDesc }))
        return promise
    }

    async *find({
        gameDesc,
        range,
        rule,
        concurrency,
    }: {
        gameDesc: Omit<GameDesc, "seed">
        range: [integer, integer]
        rule: Rule
        concurrency: integer
    }): AsyncGenerator<Galaxy, any, undefined> {
        const ws = await connect()
        let results: Galaxy[] = []
        let done = false
        let resolve: () => void = () => {}
        let promise = new Promise<void>((r) => {
            resolve = r
        })

        this._stop = () => {
            if (ws.readyState === WebSocket.OPEN) {
                ws.send(JSON.stringify({ type: "Stop" }))
            }
        }

        ws.addEventListener("close", () => {
            done = true
            resolve()
        })

        ws.addEventListener("message", (ev) => {
            if (!ev.data) {
                done = true
                resolve()
            } else {
                results.push(JSON.parse(ev.data))
                resolve()
                promise = new Promise<void>((r) => {
                    resolve = r
                })
            }
        })

        ws.send(
            JSON.stringify({
                type: "Find",
                game: { ...gameDesc, seed: 0 },
                range,
                rule,
                concurrency,
            }),
        )

        while (!done) {
            await promise
            yield* results
            results = []
        }
        ws.close()
    }

    stop() {
        this._stop()
        this._stop = () => {}
    }
}
