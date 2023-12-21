import { TinyEmitter } from "tiny-emitter"
import WorldgenWorker from "./worldgen.worker?worker"

const GENERATE_NAME = "generate"
const FREE_EVENT = "free"

export class WorldGenImpl implements WorldGen {
    private readonly workerPool: Worker[] = []
    private readonly runningWorkers = new WeakSet<Worker>()
    concurrency: number = Math.max(navigator.hardwareConcurrency, 1)
    private readonly emitter = new TinyEmitter()
    private destroyed = false

    private clearExtraWorkers() {
        while (this.workerPool.length > this.concurrency) {
            const idleWorkerIndex = this.workerPool.findLastIndex(
                (worker) => !this.runningWorkers.has(worker),
            )
            if (idleWorkerIndex > -1) {
                this.workerPool[idleWorkerIndex]?.terminate()
                this.workerPool.splice(idleWorkerIndex, 1)
            } else {
                break
            }
        }
    }

    private assertNotDestroyed() {
        if (this.destroyed) {
            throw new Error("Cannot use a destroyed WorldGen")
        }
    }

    private getAvailableWorker() {
        if (this.destroyed) return null
        this.clearExtraWorkers()
        for (const worker of this.workerPool) {
            if (!this.runningWorkers.has(worker)) {
                return worker
            }
        }
        if (this.workerPool.length < this.concurrency) {
            const worker = new WorldgenWorker()
            this.workerPool.push(worker)
            return worker
        }

        return null
    }

    private async useAvailableWorker<T>(
        cb: (worker: Worker) => Promise<T>,
    ): Promise<T> {
        this.assertNotDestroyed()
        let worker = this.getAvailableWorker()
        while (!worker) {
            await new Promise((resolve) => {
                this.emitter.once(FREE_EVENT, resolve)
            })
            this.assertNotDestroyed()
            worker = this.getAvailableWorker()
        }
        this.runningWorkers.add(worker)
        try {
            const result = await cb(worker)
            return result
        } finally {
            this.runningWorkers.delete(worker)
        }
    }

    private async doWork(
        worker: Worker,
        type: string,
        input: any,
    ): Promise<any> {
        return new Promise((resolve) => {
            const eventHandler = (ev: MessageEvent) => {
                const message = ev.data
                if (message.type === type) {
                    worker.removeEventListener("message", eventHandler)
                    resolve(message.data)
                    if (this.destroyed) {
                        worker.terminate()
                    } else {
                        this.emitter.emit(FREE_EVENT)
                    }
                }
            }
            worker.addEventListener("message", eventHandler)
            worker.postMessage({ type, input })
        })
    }

    async generate(gameDesc: GameDesc): Promise<Galaxy> {
        return await this.useAvailableWorker((worker) => {
            return this.doWork(worker, GENERATE_NAME, gameDesc)
        })
    }

    find(
        gameDesc: Omit<GameDesc, "seed">,
        rule: Rule,
    ): AsyncIterator<Galaxy, any, undefined> {
        throw new Error("Method not implemented.")
    }

    destroy() {
        this.destroyed = true
        this.concurrency = 0
        this.emitter.emit(FREE_EVENT)
        this.clearExtraWorkers()
        this.workerPool.length = 0
    }
}
