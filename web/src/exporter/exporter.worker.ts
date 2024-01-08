import {
    distanceFromBirth,
    furthestDistanceFrom,
    gasOrder,
    getStarType,
    nearestDistanceFrom,
    planetTypes,
    romans,
    statVein,
    veinOrder,
} from "../util"
import { GasType, OceanType, VeinType } from "../enums"
import {
    PlanetField,
    StarField,
    planetFieldsOrder,
    starFieldsOrder,
} from "./common"
import init, { generate } from "worldgen-wasm"

const initPromise = init()

const starFieldsGetter: Partial<
    Record<StarField, (galaxy: Galaxy, star: Star) => any>
> = {
    [StarField.Seed]: (galaxy) => galaxy.seed,
    [StarField.Index]: (galaxy, star) => star.index + 1,
    [StarField.Name]: (galaxy, star) => star.name,
    [StarField.PositionX]: (galaxy, star) => star.position[0],
    [StarField.PositionY]: (galaxy, star) => star.position[1],
    [StarField.PositionZ]: (galaxy, star) => star.position[2],
    [StarField.Mass]: (galaxy, star) => star.mass,
    [StarField.Age]: (galaxy, star) => star.age * star.lifetime,
    [StarField.Temperature]: (galaxy, star) => star.temperature,
    [StarField.Type]: (galaxy, star) => getStarType(star),
    [StarField.Spectr]: (galaxy, star) => star.spectr,
    [StarField.Luminosity]: (galaxy, star) => star.luminosity,
    [StarField.Radius]: (galaxy, star) => star.radius * 1600,
    [StarField.DysonRadius]: (galaxy, star) => star.dysonRadius,
    [StarField.DistanceFromBirth]: (galaxy, star) =>
        distanceFromBirth(star.position),
}

const planetFieldsGetter: Partial<
    Record<PlanetField, (galaxy: Galaxy, star: Star, planet: Planet) => any>
> = {
    [PlanetField.Seed]: (galaxy) => galaxy.seed,
    [PlanetField.Index]: (galaxy, star) => star.index + 1,
    [PlanetField.Name]: (galaxy, star, planet) =>
        `${star.name} ${romans[planet.index]}`,
    [PlanetField.Theme]: (galaxy, star, planet) => planetTypes[planet.theme.id],
    [PlanetField.Orbiting]: (galaxy, star, planet) =>
        planet.orbitAround != null
            ? `${star.name} ${romans[star.planets[planet.orbitAround]!.index]}`
            : "",
    [PlanetField.TidallyLocked]: (galaxy, star, planet) =>
        planet.orbitalPeriod === planet.rotationPeriod,
    [PlanetField.Wind]: (galaxy, star, planet) => planet.theme.wind * 100,
    [PlanetField.Luminosity]: (galaxy, star, planet) => planet.luminosity * 100,
    [PlanetField.OrbitRadius]: (galaxy, star, planet) => planet.orbitRadius,
    [PlanetField.OrbitInclination]: (galaxy, star, planet) =>
        planet.orbitInclination,
    [PlanetField.OrbitLongitude]: (galaxy, star, planet) =>
        planet.orbitLongitude,
    [PlanetField.OrbitalPeriod]: (galaxy, star, planet) => planet.orbitalPeriod,
    [PlanetField.OrbitPhase]: (galaxy, star, planet) => planet.orbitPhase,
    [PlanetField.Obliquity]: (galaxy, star, planet) => planet.obliquity,
    [PlanetField.RotationPeriod]: (galaxy, star, planet) =>
        planet.rotationPeriod,
    [PlanetField.RotationPhase]: (galaxy, star, planet) => planet.rotationPhase,
}

function normalizeVein(vein: VeinStat): VeinStat {
    if (vein.veinType === VeinType.Oil) {
        return {
            veinType: VeinType.Oil,
            min: vein.min * 4e-5,
            max: vein.max * 4e-5,
            avg: vein.avg * 4e-5,
        }
    } else {
        return vein
    }
}

function constructStarData(galaxy: Galaxy, star: Star) {
    const output: any[] = []
    const positions = galaxy.stars.map((x) => x.position)
    for (const field of starFieldsOrder) {
        switch (field) {
            case StarField.DistanceFromNearestX:
                output.push(nearestDistanceFrom(star.position, positions))
                break
            case StarField.DistanceFromFurthestX:
                output.push(furthestDistanceFrom(star.position, positions))
                break
            default:
                output.push(starFieldsGetter[field]?.(galaxy, star))
                break
        }
    }
    const veins: Partial<Record<VeinType, VeinStat>> = {}
    for (const planet of star.planets) {
        for (const vein of planet.veins) {
            const stat = normalizeVein(statVein(vein))
            const existing = veins[vein.veinType]
            if (existing) {
                existing.min += stat.min
                existing.max += stat.max
                existing.avg += stat.avg
            } else {
                veins[vein.veinType] = stat
            }
        }
    }
    for (const type of veinOrder) {
        output.push(veins[type]?.avg ?? 0)
        output.push(veins[type]?.min ?? 0)
        output.push(veins[type]?.max ?? 0)
    }
    output.push(
        !!star.planets.find(
            (planet) => planet.theme.waterItemId === OceanType.Water,
        ),
    )
    output.push(
        !!star.planets.find(
            (planet) => planet.theme.waterItemId === OceanType.Sulfur,
        ),
    )
    const gases: Partial<Record<GasType, float>> = {}
    for (const planet of star.planets) {
        for (const [type, amount] of planet.gases) {
            gases[type] ??= 0
            gases[type]! += amount
        }
    }
    for (const type of gasOrder) {
        output.push(gases[type] ?? 0)
    }
    return output
}

function constructPlanetData(galaxy: Galaxy, star: Star, planet: Planet) {
    const output: any[] = []
    for (const field of planetFieldsOrder) {
        output.push(planetFieldsGetter[field]?.(galaxy, star, planet))
    }
    const veins: Partial<Record<VeinType, VeinStat>> = {}
    for (const vein of planet.veins) {
        veins[vein.veinType] = normalizeVein(statVein(vein))
    }
    for (const type of veinOrder) {
        output.push(veins[type]?.avg ?? 0)
        output.push(veins[type]?.min ?? 0)
        output.push(veins[type]?.max ?? 0)
    }
    output.push(planet.theme.waterItemId === OceanType.Water)
    output.push(planet.theme.waterItemId === OceanType.Sulfur)
    const gases: Partial<Record<GasType, float>> = {}
    for (const [type, amount] of planet.gases) {
        gases[type] = amount
    }
    for (const type of gasOrder) {
        output.push(gases[type] ?? 0)
    }
    return output
}

function generateExportData(
    galaxy: Galaxy,
    indexes: integer[],
    exportAllStars: boolean,
): ExportData {
    let stars = galaxy.stars
    if (!exportAllStars) {
        stars = stars.filter((s) => indexes.includes(s.index))
    }
    return {
        seed: galaxy.seed,
        stars: stars.map((star) => constructStarData(galaxy, star)),
        planets: stars.flatMap((star) =>
            star.planets.map((planet) =>
                constructPlanetData(galaxy, star, planet),
            ),
        ),
    }
}

self.onmessage = (ev) => {
    const {
        seed,
        indexes,
        resourceMultiplier = 1,
        starCount = 64,
        exportAllStars,
    } = ev.data

    initPromise.then(() => {
        const result = generate({
            seed,
            starCount,
            resourceMultiplier,
            until: exportAllStars ? null : indexes.reduce(Math.max, -1) + 1,
        })
        const data = generateExportData(result, indexes, exportAllStars)
        self.postMessage(data)
    })
}
