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
    console.debug("connecting")
    await waitConnect(ws)
    console.debug("connected")
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

    find({
        gameDesc,
        range,
        rule,
        concurrency,
        autosave,
        onError,
        onResult,
        onProgress,
        onComplete,
        onInterrupt,
    }: FindOptions) {
        connect()
            .then((ws) => {
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
                    if (msg.type === "Result") {
                        onResult?.({ seed: msg.seed, indexes: msg.indexes })
                    } else {
                        onProgress?.(msg.end)
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
                        game: gameDesc,
                        range,
                        rule,
                        concurrency,
                        autosave,
                    }),
                )
            })
            .catch((err) => onError?.(err))
    }

    stop() {
        this._stop()
        this._stop = () => {}
    }
}
