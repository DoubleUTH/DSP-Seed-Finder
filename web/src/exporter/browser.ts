import ExporterManager from "./exporterManager.worker?worker"

export const browserExportGalaxies: Exporter = async (options) => {
    const { onProgress, onGenerate, ...rest } = options
    const manager = new ExporterManager()
    return new Promise<Blob | null>((resolve) => {
        manager.addEventListener("message", (ev) => {
            const { type, current, result } = ev.data
            if (type === "progressing") {
                const stopped = onProgress(current)
                if (stopped) {
                    resolve(null)
                    manager.terminate()
                }
            } else if (type === "generating") {
                onGenerate()
            } else {
                const blob = new Blob([result], {
                    type:
                        rest.format === "xlsx"
                            ? "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                            : "application/zip",
                })
                resolve(blob)
                manager.terminate()
            }
        })
        manager.postMessage(rest)
    })
}
