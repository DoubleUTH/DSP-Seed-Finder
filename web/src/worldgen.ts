import WorldgenWorker from "./worldgen.worker?worker"
import { TinyEmitter } from "tiny-emitter"

const workerPool: Worker[] = []
const runningWorkers = new WeakSet<Worker>()
let workerCount = 0
const emitter = new TinyEmitter()
const PING_NAME = "PING"

export function setWorkerCount(count: integer) {
    workerCount = count
}

function getAvailableWorker() {
    if (workerCount < 1) {
        workerCount = Math.max(1, 1)
    }
    for (const worker of workerPool) {
        if (!runningWorkers.has(worker)) {
            return worker
        }
    }
    if (workerPool.length < workerCount) {
        const worker = new WorldgenWorker()
        workerPool.push(worker)
        return worker
    }

    return null
}

export async function generateGalaxy(game: GameDesc): Promise<Galaxy> {
    function useWorker(worker: Worker): Promise<Galaxy> {
        return new Promise((resolve) => {
            function eventHandler(ev: MessageEvent) {
                const galaxy = JSON.parse(ev.data)
                worker.removeEventListener("message", eventHandler)
                runningWorkers.delete(worker)
                resolve(galaxy)
                emitter.emit(PING_NAME)
            }
            runningWorkers.add(worker)
            worker.addEventListener("message", eventHandler)
            worker.postMessage(game)
        })
    }

    let worker = getAvailableWorker()
    while (!worker) {
        await new Promise((resolve) => emitter.once(PING_NAME, resolve))
        worker = getAvailableWorker()
    }

    return useWorker(worker)
}
