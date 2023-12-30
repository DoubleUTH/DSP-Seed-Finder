import { customAlphabet } from "nanoid"

const databases = new Map<string, Promise<IDBDatabase>>()
const nanoid = customAlphabet(
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
    20,
)

const PREFIX = "profile_"
const SETTINGS = "settings"
const STARS = "stars"

export function generateDatatabaseId(): string {
    return PREFIX + nanoid()
}

export async function getDatabases(): Promise<string[]> {
    const list = await indexedDB.databases()
    return list.map((v) => v.name!).filter((name) => name.startsWith(PREFIX))
}

async function openDatabase(id: string): Promise<IDBDatabase> {
    const exist = databases.get(id)
    if (exist) return exist
    const request = indexedDB.open(id, 1)
    const promise = new Promise<IDBDatabase>((resolve, reject) => {
        request.onblocked = (ev) => console.error(ev)
        request.onupgradeneeded = (ev) => {
            const db: IDBDatabase = (ev.target as any).result
            db.createObjectStore(SETTINGS, { keyPath: "id" })
            db.createObjectStore(STARS, { keyPath: "id" })
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

export async function getProfileSettings(
    id: string,
): Promise<ProfileSettings | null> {
    const db = await openDatabase(id)
    const txn = db.transaction([SETTINGS], "readonly")
    const store = txn.objectStore(SETTINGS)
    const req = store.get(id)

    return new Promise((resolve, reject) => {
        txn.onerror = reject
        req.onerror = reject
        req.onsuccess = () => {
            resolve(req.result || null)
        }
    })
}

export async function setProfileSettings(
    settings: ProfileSettings,
): Promise<void> {
    const db = await openDatabase(settings.id)
    const txn = db.transaction([SETTINGS], "readwrite")
    const store = txn.objectStore(SETTINGS)
    store.put(settings)

    await new Promise((resolve, reject) => {
        txn.onerror = reject
        txn.oncomplete = resolve
    })
}

export async function saveToProfile(
    id: string,
    currentSeed: integer,
    results: FindResult[] = [],
) {
    const db = await openDatabase(id)
    const txn = db.transaction(
        results.length > 0 ? [SETTINGS, STARS] : [SETTINGS],
        "readwrite",
    )
    await new Promise((resolve, reject) => {
        txn.oncomplete = resolve
        txn.onerror = reject

        const settingsStore = txn.objectStore(SETTINGS)
        const req = settingsStore.get(id)
        req.onsuccess = () => {
            console.log(req.result)
            settingsStore.put({ ...req.result, current: currentSeed })
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
