import { Workbook } from "exceljs"
import ExporterWorker from "./exporter.worker?worker"
import {
    PlanetField,
    planetFieldsOrder,
    StarField,
    starFieldsOrder,
} from "./common"
import { t } from "#linguiCore"
import { loadLanguage } from "../linguiCore"
import { VeinType, GasType } from "../enums"
import { gasOrder, veinOrder } from "../util"

function getVeinFieldsOrder(): string[] {
    const veinNames: Record<VeinType, string> = {
        [VeinType.None]: "",
        [VeinType.Iron]: t`Iron Ore`,
        [VeinType.Copper]: t`Copper Ore`,
        [VeinType.Silicium]: t`Silicon Ore`,
        [VeinType.Titanium]: t`Titanium Ore`,
        [VeinType.Stone]: t`Stone`,
        [VeinType.Coal]: t`Coal`,
        [VeinType.Oil]: t`Crude Oil`,
        [VeinType.Fireice]: t`Fire Ice`,
        [VeinType.Diamond]: t`Kimberlite Ore`,
        [VeinType.Fractal]: t`Fractal Silicon`,
        [VeinType.Crysrub]: t`Organic Crystal`,
        [VeinType.Grat]: t`Grating Crystal`,
        [VeinType.Bamboo]: t`Stalagmite Crystal`,
        [VeinType.Mag]: t`Unipolar Magnet`,
    }

    const gasNames: Record<GasType, string> = {
        [GasType.None]: "",
        [GasType.Fireice]: t`Fire Ice`,
        [GasType.Hydrogen]: t`Hydrogen`,
        [GasType.Deuterium]: t`Deuterium`,
    }

    const veinFieldsOrder = [
        ...veinOrder.flatMap((type) => {
            const name = veinNames[type]
            return [t`${name} (Avg)`, t`${name} (Min)`, t`${name} (Max)`]
        }),
        t`Water`,
        t`Sulfuric Acid`,
        ...gasOrder.map((type) => gasNames[type]),
    ]
    return veinFieldsOrder
}

function getStarFieldNames(): Record<StarField, string> {
    return {
        [StarField.Seed]: t`Seed`,
        [StarField.Index]: t`Index`,
        [StarField.Name]: t`Name`,
        [StarField.PositionX]: t`X`,
        [StarField.PositionY]: t`Y`,
        [StarField.PositionZ]: t`Z`,
        [StarField.Mass]: t`Mass`,
        [StarField.Age]: t`Age`,
        [StarField.Temperature]: t`Temperature`,
        [StarField.Type]: t`Type`,
        [StarField.Spectr]: t`Spectral class`,
        [StarField.Luminosity]: t`Luminosity`,
        [StarField.Radius]: t`Radius`,
        [StarField.DysonRadius]: t`Max dyson sphere radius`,
        [StarField.DistanceFromBirth]: t`Distance from start`,
        [StarField.DistanceFromNearestX]: t`Distance from nearest X star`,
        [StarField.DistanceFromFurthestX]: t`Distance from furthest X star`,
        [StarField.InitialHiveCount]: t`Initial number of hives`,
        [StarField.MaxHiveCount]: t`Maximum number of hives`,
    }
}

function getPlanetFieldNames(): Record<PlanetField, string> {
    return {
        [PlanetField.Seed]: t`Seed`,
        [PlanetField.Index]: t`Index`,
        [PlanetField.Name]: t`Name`,
        [PlanetField.Theme]: t`Theme`,
        [PlanetField.Orbiting]: t`Orbiting`,
        [PlanetField.TidallyLocked]: t`Tidally locked`,
        [PlanetField.OrbitRadius]: t`Orbit radius`,
        [PlanetField.OrbitInclination]: t`Orbit inclination`,
        [PlanetField.OrbitLongitude]: t`Orbit longitude`,
        [PlanetField.OrbitalPeriod]: t`Orbital period`,
        [PlanetField.Obliquity]: t`Obliquity`,
        [PlanetField.RotationPeriod]: t`Rotation period`,
        [PlanetField.Wind]: t`Wind power`,
        [PlanetField.Luminosity]: t`Solar power`,
    }
}

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
            const starFieldNames = getStarFieldNames()
            const planetFieldNames = getPlanetFieldNames()
            const veinFieldsOrder = getVeinFieldsOrder()
            const starsSheet = book.addWorksheet("Stars", {
                views: [{ state: "frozen", ySplit: 1 }],
            })
            starsSheet.addRow([
                ...starFieldsOrder.map((f) => starFieldNames[f]),
                ...veinFieldsOrder,
            ])
            const planetsSheet = book.addWorksheet("Planets", {
                views: [{ state: "frozen", ySplit: 1 }],
            })
            planetsSheet.addRow([
                ...planetFieldsOrder.map((f) => planetFieldNames[f]),
                ...veinFieldsOrder,
            ])
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
        language,
    } = options
    await loadLanguage(language)
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
                language,
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
