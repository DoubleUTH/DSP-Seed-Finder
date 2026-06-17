import { Workbook } from "exceljs"
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

function getVeinFieldsOrder(useActualVeins: boolean): string[] {
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
            return useActualVeins
                ? [name]
                : [t`${name} (Avg)`, t`${name} (Min)`, t`${name} (Max)`]
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

function createWorkbook(useActualVeins: boolean) {
    const book = new Workbook()
    const starFieldNames = getStarFieldNames()
    const planetFieldNames = getPlanetFieldNames()
    const veinFieldsOrder = getVeinFieldsOrder(useActualVeins)
    const starsSheet = book.addWorksheet("Stars", {
        views: [{ state: "frozen", xSplit: 3, ySplit: 1 }],
    })
    starsSheet.addRow([
        ...starFieldsOrder.map((f) => starFieldNames[f]),
        ...veinFieldsOrder,
    ])
    const planetsSheet = book.addWorksheet("Planets", {
        views: [{ state: "frozen", xSplit: 3, ySplit: 1 }],
    })
    planetsSheet.addRow([
        ...planetFieldsOrder.map((f) => planetFieldNames[f]),
        ...veinFieldsOrder,
    ])

    return {
        add(data: ExportData) {
            starsSheet.addRows(data.stars)
            planetsSheet.addRows(data.planets)
        },
        async buffer(format: ExportOptions["format"]): Promise<Uint8Array> {
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
                const output = await zip.generateAsync({ type: "uint8array" })
                return output
            } else {
                const buffer =
                    (await book.xlsx.writeBuffer()) as unknown as Uint8Array
                return buffer
            }
        },
    }
}

let loadPromise: Promise<ReturnType<typeof createWorkbook>> | undefined

self.onmessage = (ev) => {
    if (loadPromise) {
        loadPromise.then((workbook) => {
            if (typeof ev.data === "string") {
                const format = ev.data as any
                workbook.buffer(format).then((result) => {
                    self.postMessage(result, [result.buffer])
                })
            } else {
                workbook.add(ev.data)
            }
        })
    } else {
        loadPromise = loadLanguage(ev.data.language).then(() =>
            createWorkbook(ev.data.useActualVeins),
        )
    }
}
