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
                resolve(JSON.parse(ev.data).galaxy)
                ws.close()
            })
        })
        ws.send(JSON.stringify({ type: "Generate", game: gameDesc }))
        return promise
    }

    find({
        gameDesc,
        range,
        rule,
        concurrency,
        onProgress,
        onComplete,
        onInterrupt,
    }: {
        gameDesc: Omit<GameDesc, "seed">
        range: [integer, integer]
        rule: Rule
        concurrency: integer
        onProgress?: (current: number, galaxys: Galaxy[]) => void
        onComplete?: () => void
        onInterrupt?: () => void
    }) {
        connect().then((ws) => {
            let results: Galaxy[] = []
            let done = false

            this._stop = () => {
                if (ws.readyState === WebSocket.OPEN) {
                    ws.send(JSON.stringify({ type: "Stop" }))
                }
            }

            ws.addEventListener("close", () => {
                if (!done) {
                    onInterrupt?.()
                }
            })

            ws.addEventListener("message", (ev) => {
                const msg = JSON.parse(ev.data)
                if (msg.type === "Galaxy") {
                    results.push(msg.galaxy)
                } else {
                    onProgress?.(msg.end, results)
                    results = []
                    if (msg.type === "Done") {
                        done = true
                        onComplete?.()
                        ws.close()
                    }
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
        })
    }

    stop() {
        this._stop()
        this._stop = () => {}
    }
}
