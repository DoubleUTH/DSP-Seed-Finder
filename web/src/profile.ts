import { customAlphabet } from "nanoid"

const databases = new Map<string, Promise<IDBDatabase>>()
const nanoid = customAlphabet(
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
    6,
)

const PREFIX = "p_"
const INFO = "info"
const INFO_MULTI = "info_multi"
const PROGRESS = "progress"
const STARS = "stars"
const GALAXIES = "galaxies"

export function generateProfileId(): string {
    return PREFIX + Date.now() + nanoid()
}

async function openDatabase(
    id: string,
    onUpgrade: (db: IDBDatabase) => void,
): Promise<IDBDatabase> {
    const exist = databases.get(id)
    if (exist) return exist
    const request = indexedDB.open(id, 1)
    const promise = new Promise<IDBDatabase>((resolve, reject) => {
        request.onblocked = (ev) => console.error(ev)
        request.onupgradeneeded = (ev) => {
            const db: IDBDatabase = (ev.target as any).result
            onUpgrade(db)
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

function upgradeInfoDb(db: IDBDatabase) {
    db.createObjectStore(INFO, { keyPath: "id" })
}

function upgradeProfileDb(db: IDBDatabase) {
    db.createObjectStore(PROGRESS, { keyPath: "id" })
    db.createObjectStore(STARS, { keyPath: "id" })
}

function upgradeMultiProfileDb(db: IDBDatabase) {
    db.createObjectStore(PROGRESS, { keyPath: "id" })
    db.createObjectStore(GALAXIES, { keyPath: "seed" })
}

function openInfoDatabase() {
    return openDatabase(INFO, upgradeInfoDb)
}

function openMultiInfoDatabase() {
    return openDatabase(INFO_MULTI, upgradeInfoDb)
}

function openProfileDatabase(id: string) {
    return openDatabase(id, upgradeProfileDb)
}

function openMultiProfileDatabase(id: string) {
    return openDatabase(id, upgradeMultiProfileDb)
}

export async function listProfiles(): Promise<ProfileInfo[]> {
    const db = await openInfoDatabase()
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
    const db = await openInfoDatabase()
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
    const db = await openInfoDatabase()
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
    const db = await openProfileDatabase(id)
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
    const db = await openProfileDatabase(progress.id)
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
    const db = await openProfileDatabase(id)
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
    const db = await openInfoDatabase()
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
    const db = await openProfileDatabase(id)
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

export async function getProfileResultRange(
    id: string,
    start: number,
    end: number,
): Promise<ProgressResult[]> {
    const db = await openProfileDatabase(id)
    const txn = db.transaction([STARS], "readonly")
    const store = txn.objectStore(STARS)
    const req = store.getAll(IDBKeyRange.bound(start * 100, end * 100 + 99))
    return new Promise((resolve, reject) => {
        txn.onerror = reject
        req.onerror = reject
        req.onsuccess = () => {
            resolve(req.result)
        }
    })
}

export async function listMultiProfiles(): Promise<ProfileInfo[]> {
    const db = await openMultiInfoDatabase()
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

export async function getMultiProfileInfo(
    id: string,
): Promise<ProfileInfo | null> {
    const db = await openMultiInfoDatabase()
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

export async function setMultiProfileInfo(info: ProfileInfo): Promise<void> {
    const db = await openMultiInfoDatabase()
    const txn = db.transaction([INFO], "readwrite")
    const store = txn.objectStore(INFO)
    store.put(info)

    await new Promise((resolve, reject) => {
        txn.onerror = reject
        txn.oncomplete = resolve
    })
}

export async function getMultiProfileProgress(
    id: string,
): Promise<MultiProfileProgress | null> {
    const db = await openMultiProfileDatabase(id)
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

export async function setMultiProfileProgress(
    progress: MultiProfileProgress,
    results: FindResult[] = [],
) {
    const db = await openMultiProfileDatabase(progress.id)
    const txn = db.transaction(
        results.length > 0 ? [PROGRESS, GALAXIES] : [PROGRESS],
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
            const store = txn.objectStore(GALAXIES)
            results.forEach((result) => {
                store.put({
                    seed: result.seed,
                })
            })
        }
    })
}

export async function clearMultiProfile(id: string) {
    const db = await openMultiProfileDatabase(id)
    const txn = db.transaction([PROGRESS, GALAXIES], "readwrite")

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

    const store = txn.objectStore(GALAXIES)
    store.clear()

    await new Promise((resolve, reject) => {
        txn.oncomplete = resolve
        txn.onerror = reject
    })
}

export async function deleteMultiProfile(id: string) {
    const conn = databases.get(id)
    if (conn) {
        const db = await conn
        db.close()
        databases.delete(id)
    }
    const deleteRequest = indexedDB.deleteDatabase(id)
    const db = await openMultiInfoDatabase()
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

export async function getMultiProfileResult(
    id: string,
    start: number,
    count: number,
) {
    const db = await openMultiProfileDatabase(id)
    const txn = db.transaction([GALAXIES], "readonly")
    const store = txn.objectStore(GALAXIES)
    const cursor = store.openCursor()
    let advanced = false
    const results: MultiProgressResult[] = []
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

    return new Promise<MultiProgressResult[]>((resolve, reject) => {
        txn.onerror = reject
        txn.oncomplete = () => {
            resolve(results)
        }
        cursor.onerror = reject
    })
}

export async function getMultiProfileResultRange(
    id: string,
    start: number,
    end: number,
): Promise<MultiProgressResult[]> {
    const db = await openMultiProfileDatabase(id)
    const txn = db.transaction([GALAXIES], "readonly")
    const store = txn.objectStore(GALAXIES)
    const req = store.getAll(IDBKeyRange.bound(start, end))
    return new Promise((resolve, reject) => {
        txn.onerror = reject
        req.onerror = reject
        req.onsuccess = () => {
            resolve(req.result)
        }
    })
}
