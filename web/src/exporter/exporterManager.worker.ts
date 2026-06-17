import ExporterWorker from "./exporter.worker?worker"
import GeneratorWorker from "./workbookGenerator.worker?worker"

async function go(options: ExportOptions) {
    const { format, concurrency, results, params, language } = options
    const threads = Math.max(1, Math.min(concurrency - 1, results.length))
    let index = 0
    let count = 0
    let running = threads
    let end = () => {}
    const generator = new GeneratorWorker()
    generator.postMessage({ language, useActualVeins: params.useActualVeins })
    for (let i = 0; i < threads; ++i) {
        const worker = new ExporterWorker()
        const stop = () => {
            worker.terminate()
            if (--running === 0) {
                end()
            }
        }
        const sendNext = () => {
            const item = results[index++]
            if (!item) {
                stop()
                return
            }
            worker.postMessage(item)
        }
        worker.addEventListener("message", (ev) => {
            generator.postMessage(ev.data)
            self.postMessage({ type: "progressing", current: ++count })
            sendNext()
        })
        worker.postMessage({
            params,
            language,
        })
        sendNext()
    }
    await new Promise<void>((resolve) => {
        end = resolve
    })
    self.postMessage({ type: "generating" })
    generator.onmessage = (ev) => {
        self.postMessage({ type: "done", result: ev.data }, [ev.data.buffer])
    }
    generator.postMessage(format)
}

self.onmessage = (ev) => {
    go(ev.data)
}
