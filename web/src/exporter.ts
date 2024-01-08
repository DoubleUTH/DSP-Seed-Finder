import { browserExportGalaxies } from "./exporter/browser"

export function getExporter(nativeMode: boolean): Exporter {
    if (nativeMode) {
        throw new Error("Exporter in native mode is not implemented")
    }
    return browserExportGalaxies
}
