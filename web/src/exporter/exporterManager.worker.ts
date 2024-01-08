import { Workbook } from "exceljs"
import ExporterWorker from "./exporter.worker?worker"
import { planetFieldsOrder, starFieldsOrder, veinFieldsOrder } from "./common"

function createWorkbook() {
    let map: Record<number, ExportData> = {}

    return {
        add(data: ExportData) {
            map[data.seed] = data
        },
        async buffer(
            format: ExportOptions["format"],
            seeds: FindResult[],
            exportAllStars: boolean,
        ): Promise<ArrayBuffer> {
            const book = new Workbook()
            const starsSheet = book.addWorksheet("Stars", {
                views: [{ state: "frozen", ySplit: 1 }],
            })
            starsSheet.addRow([...starFieldsOrder, ...veinFieldsOrder])
            const planetsSheet = book.addWorksheet("Planets", {
                views: [{ state: "frozen", ySplit: 1 }],
            })
            planetsSheet.addRow([...planetFieldsOrder, ...veinFieldsOrder])
            for (const { seed, indexes } of seeds) {
                const { stars } = map[seed]!
                const rows = starsSheet.addRows(stars)
                if (exportAllStars && indexes) {
                    for (const index of indexes) {
                        rows[index]!.font = {
                            bold: true,
                        }
                    }
                }
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
                const output = await zip.generateAsync({ type: "arraybuffer" })
                return output
            } else {
                const buffer = await book.xlsx.writeBuffer()
                return buffer
            }
        },
    }
}

async function go(options: ExportOptions) {
    const {
        format,
        concurrency,
        exportAllStars,
        results,
        starCount,
        resourceMultiplier,
    } = options
    const workbook = createWorkbook()
    const threads = Math.min(concurrency, results.length)
    let index = 0
    let count = 0
    let running = threads
    let end = () => {}
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
            worker.postMessage({
                ...item,
                starCount,
                resourceMultiplier,
                exportAllStars,
            })
        }
        worker.addEventListener("message", (ev) => {
            workbook.add(ev.data)
            self.postMessage({ type: "progressing", current: ++count })
            sendNext()
        })
        sendNext()
    }
    await new Promise<void>((resolve) => {
        end = resolve
    })
    self.postMessage({ type: "generating" })
    const result = await workbook.buffer(format, results, exportAllStars)
    self.postMessage({ type: "done", result }, [
        format === "xlsx" ? (result as any).buffer : result,
    ])
}

self.onmessage = (ev) => {
    go(ev.data)
}
