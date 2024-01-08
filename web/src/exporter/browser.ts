import { Workbook } from "exceljs"
import ExporterWorker from "./exporter.worker?worker"
import { planetFieldsOrder, starFieldsOrder, veinFieldsOrder } from "./common"
import { TinyEmitter } from "tiny-emitter"

function createWorkbook() {
    let map: Record<number, ExportData> = {}

    return {
        add(data: ExportData) {
            map[data.seed] = data
        },
        async blob(
            format: ExportOptions["format"],
            seeds: FindResult[],
        ): Promise<Blob> {
            const book = new Workbook()
            const starsSheet = book.addWorksheet("Stars", {
                views: [{ state: "frozen", ySplit: 1 }],
            })
            starsSheet.addRow([...starFieldsOrder, ...veinFieldsOrder])
            for (const { seed, indexes } of seeds) {
                const { stars } = map[seed]!
                const rows = starsSheet.addRows(stars)
                for (const index of indexes) {
                    rows[index]!.font = {
                        bold: true,
                    }
                }
            }
            const planetsSheet = book.addWorksheet("Planets", {
                views: [{ state: "frozen", ySplit: 1 }],
            })
            planetsSheet.addRow([...planetFieldsOrder, ...veinFieldsOrder])
            for (const { seed } of seeds) {
                const { planets } = map[seed]!
                planetsSheet.addRows(planets)
            }
            // clear memory after write
            map = {}

            if (format === "csv") {
                const JSZip = (await import("jszip")).default
                const zip = new JSZip()
                zip.file(
                    "stars.csv",
                    book.csv.writeBuffer({ sheetId: starsSheet.id }),
                )
                zip.file(
                    "planets.csv",
                    book.csv.writeBuffer({ sheetId: planetsSheet.id }),
                )
                const output = await zip.generateAsync({ type: "blob" })
                return output
            } else {
                const buffer = await book.xlsx.writeBuffer()
                return new Blob([buffer], {
                    type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                })
            }
        },
    }
}

export const browserExportGalaxies: Exporter = async (options) => {
    const { format, concurrency, exportAllStars, results, onProgress } = options
    const emitter = new TinyEmitter()
    const workbook = createWorkbook()
    const threads = Math.min(concurrency, results.length)
    let index = 0
    let count = 0
    let running = threads
    for (let i = 0; i < threads; ++i) {
        const worker = new ExporterWorker()
        const sendNext = () => {
            const item = results[index++]
            if (!item) {
                worker.terminate()
                if (--running === 0) {
                    emitter.emit("end")
                }
                return
            }
            worker.postMessage({
                ...item,
                starCount: options.starCount,
                resourceMultiplier: options.resourceMultiplier,
                exportAllStars,
            })
        }
        worker.addEventListener("message", (ev) => {
            workbook.add(ev.data)
            onProgress(++count)
            sendNext()
        })
        sendNext()
    }
    await new Promise<void>((resolve) => {
        emitter.once("end", resolve)
    })
    return () => workbook.blob(format, results)
}
