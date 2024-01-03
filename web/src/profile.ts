import { customAlphabet } from "nanoid"

const databases = new Map<string, Promise<IDBDatabase>>()
const nanoid = customAlphabet(
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
    6,
)

const PREFIX = "p_"
const INFO = "info"
const PROGRESS = "progress"
const STARS = "stars"

export function generateProfileId(): string {
    return PREFIX + Date.now() + nanoid()
}

async function openDatabase(id: string): Promise<IDBDatabase> {
    const exist = databases.get(id)
    if (exist) return exist
    const request = indexedDB.open(id, 1)
    const promise = new Promise<IDBDatabase>((resolve, reject) => {
        request.onblocked = (ev) => console.error(ev)
        request.onupgradeneeded = (ev) => {
            const db: IDBDatabase = (ev.target as any).result
            if (id === INFO) {
                db.createObjectStore(INFO, { keyPath: "id" })
            } else {
                db.createObjectStore(PROGRESS, { keyPath: "id" })
                db.createObjectStore(STARS, { keyPath: "id" })
            }
        }
        request.onsuccess = () => {
            const db = request.result
            db.onclose = () => {
                databases.delete(id)
            }
            resolve(db)
        }
        request.onerror = reject
    })
    databases.set(id, promise)
    return promise
}

export async function listProfiles(): Promise<ProfileInfo[]> {
    const db = await openDatabase(INFO)
    const txn = db.transaction([INFO], "readonly")
    const store = txn.objectStore(INFO)
    const req = store.getAll()
    return new Promise((resolve, reject) => {
        txn.onerror = reject
        req.onerror = reject
        req.onsuccess = () => {
            const result = [...req.result]
            result.reverse()
            resolve(result)
        }
    })
}

export async function getProfileInfo(id: string): Promise<ProfileInfo | null> {
    const db = await openDatabase(INFO)
    const txn = db.transaction([INFO], "readonly")
    const store = txn.objectStore(INFO)
    const req = store.get(id)

    return new Promise((resolve, reject) => {
        txn.onerror = reject
        req.onerror = reject
        req.onsuccess = () => {
            resolve(req.result || null)
        }
    })
}

export async function setProfileInfo(info: ProfileInfo): Promise<void> {
    const db = await openDatabase(INFO)
    const txn = db.transaction([INFO], "readwrite")
    const store = txn.objectStore(INFO)
    store.put(info)

    await new Promise((resolve, reject) => {
        txn.onerror = reject
        txn.oncomplete = resolve
    })
}

export async function getProfileProgress(
    id: string,
): Promise<ProfileProgress | null> {
    const db = await openDatabase(id)
    const txn = db.transaction([PROGRESS], "readonly")
    const store = txn.objectStore(PROGRESS)
    const req = store.get(id)

    return new Promise((resolve, reject) => {
        txn.onerror = reject
        req.onerror = reject
        req.onsuccess = () => {
            resolve(req.result || null)
        }
    })
}

export async function setProfileProgress(
    progress: ProfileProgress,
    results: FindResult[] = [],
) {
    const db = await openDatabase(progress.id)
    const txn = db.transaction(
        results.length > 0 ? [PROGRESS, STARS] : [PROGRESS],
        "readwrite",
    )
    await new Promise((resolve, reject) => {
        txn.oncomplete = resolve
        txn.onerror = reject

        const progressStore = txn.objectStore(PROGRESS)
        const req = progressStore.get(progress.id)
        req.onsuccess = () => {
            if (!req.result || req.result.current <= progress.current) {
                progressStore.put(progress)
            }
        }

        if (results.length > 0) {
            const store = txn.objectStore(STARS)
            results.forEach((result) => {
                result.indexes.forEach((index) => {
                    store.put({
                        id: result.seed * 100 + index,
                        seed: result.seed,
                        index: index,
                    })
                })
            })
        }
    })
}

export async function clearProfile(id: string) {
    const db = await openDatabase(id)
    const txn = db.transaction([PROGRESS, STARS], "readwrite")

    const progressStore = txn.objectStore(PROGRESS)
    const req = progressStore.get(id)
    req.onsuccess = () => {
        if (req.result) {
            progressStore.put({
                ...req.result,
                current: req.result.start,
                found: 0,
            })
        }
    }

    const store = txn.objectStore(STARS)
    store.clear()

    await new Promise((resolve, reject) => {
        txn.oncomplete = resolve
        txn.onerror = reject
    })
}

export async function deleteProfile(id: string) {
    const conn = databases.get(id)
    if (conn) {
        const db = await conn
        db.close()
        databases.delete(id)
    }
    const deleteRequest = indexedDB.deleteDatabase(id)
    const db = await openDatabase(INFO)
    const txn = db.transaction([INFO], "readwrite")
    const store = txn.objectStore(INFO)
    store.delete(id)

    await new Promise<void>((resolve, reject) => {
        let count = 2
        txn.onerror = reject
        txn.oncomplete = () => {
            if (!--count) resolve()
        }
        deleteRequest.onerror = reject
        deleteRequest.onsuccess = () => {
            if (!--count) resolve()
        }
    })
}

export async function getProfileResult(
    id: string,
    start: number,
    count: number,
) {
    const db = await openDatabase(id)
    const txn = db.transaction([STARS], "readonly")
    const store = txn.objectStore(STARS)
    const cursor = store.openCursor()
    let advanced = false
    const results: ProgressResult[] = []
    cursor.onsuccess = () => {
        const result = cursor.result
        if (!result) return
        if (start > 0 && !advanced) {
            advanced = true
            cursor.result?.advance(start)
            return
        }
        results.push(result.value)
        if (results.length < count) {
            result.continue()
        }
    }

    return new Promise<ProgressResult[]>((resolve, reject) => {
        txn.onerror = reject
        txn.oncomplete = () => {
            resolve(results)
        }
        cursor.onerror = reject
    })
}
