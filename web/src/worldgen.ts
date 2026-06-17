import { WorldGenBrowser } from "./worldgen/browser"
import { WorldGenNative } from "./worldgen/native"

const browser: WorldGen = new WorldGenBrowser()
const native: WorldGen = new WorldGenNative()

const ALWAYS_NATIVE = false

function getWorldGen(nativeMode: boolean): WorldGen {
    return ALWAYS_NATIVE ? native : nativeMode ? native : browser
}

export function generateGalaxy(
    nativeMode: boolean,
    seed: integer,
    gameDesc: GameParameters,
): Promise<Galaxy> {
    return getWorldGen(nativeMode).generate(seed, gameDesc)
}

export function startSearchingGalaxies(
    nativeMode: boolean,
    options: FindOptions,
) {
    const {
        gameDesc,
        autosave,
        onProgress,
        onResult,
        onInterrupt,
        onComplete,
        onError,
        ...rest
    } = options
    const pendingBatches = new Map<integer, integer[]>()
    let { nextBatchId } = options
    const onBatchResult: InternalFindOptions["onBatchResult"] = (
        batchId,
        result,
    ) => {
        if (batchId === nextBatchId) {
            const results = [...result]
            ++nextBatchId
            while (pendingBatches.has(nextBatchId)) {
                results.push(...pendingBatches.get(nextBatchId)!)
                pendingBatches.delete(nextBatchId)
                ++nextBatchId
            }
            onResult?.(results)
        } else {
            pendingBatches.set(batchId, result)
        }
    }
    let savedBatchId = nextBatchId
    const interval = window.setInterval(() => {
        if (savedBatchId < nextBatchId) {
            savedBatchId = nextBatchId
            onProgress(nextBatchId)
        }
    }, autosave * 1000)
    let completed = false

    const done = () => {
        completed = true
        window.clearInterval(interval)
        if (savedBatchId < nextBatchId) {
            savedBatchId = nextBatchId
            onProgress(nextBatchId)
        }
    }
    getWorldGen(nativeMode)
        .find({
            ...rest,
            gameDesc: { ...gameDesc },
            onBatchResult,
            onInterrupt: () => {
                if (!completed) {
                    done()
                    onInterrupt()
                }
            },
        })
        .then(() => {
            if (!completed) {
                done()
                onComplete()
            }
        })
        .catch((err) => onError(err))
}

export function stopSearchingGalaxies(nativeMode: boolean) {
    getWorldGen(nativeMode).stop()
}
