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

function trim(number: number, precision: number) {
    const multiplier = Math.pow(10, precision)
    return Math.round(number * multiplier) / multiplier
}

const initPromise = init()

const starFieldsGetter: Partial<
    Record<StarField, (galaxy: Galaxy, star: Star) => any>
> = {
    [StarField.Seed]: (galaxy) => galaxy.seed,
    [StarField.Index]: (galaxy, star) => star.index + 1,
    [StarField.Name]: (galaxy, star) => star.name,
    [StarField.PositionX]: (galaxy, star) => trim(star.position[0], 6),
    [StarField.PositionY]: (galaxy, star) => trim(star.position[1], 6),
    [StarField.PositionZ]: (galaxy, star) => trim(star.position[2], 6),
    [StarField.Mass]: (galaxy, star) => trim(star.mass, 4),
    [StarField.Age]: (galaxy, star) => trim(star.age * star.lifetime, 4),
    [StarField.Temperature]: (galaxy, star) => trim(star.temperature, 4),
    [StarField.Type]: (galaxy, star) => getStarType(star),
    [StarField.Spectr]: (galaxy, star) => star.spectr,
    [StarField.Luminosity]: (galaxy, star) => trim(star.luminosity, 3),
    [StarField.Radius]: (galaxy, star) => trim(star.radius * 1600, 0),
    [StarField.DysonRadius]: (galaxy, star) => trim(star.dysonRadius, 0),
    [StarField.DistanceFromBirth]: (galaxy, star) =>
        trim(distanceFromBirth(star.position), 3),
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
    [PlanetField.Wind]: (galaxy, star, planet) =>
        trim(planet.theme.wind * 100, 0),
    [PlanetField.Luminosity]: (galaxy, star, planet) =>
        trim(planet.luminosity * 100, 0),
    [PlanetField.OrbitRadius]: (galaxy, star, planet) =>
        trim(planet.orbitRadius, 4),
    [PlanetField.OrbitInclination]: (galaxy, star, planet) =>
        trim(planet.orbitInclination, 4),
    [PlanetField.OrbitLongitude]: (galaxy, star, planet) =>
        trim(planet.orbitLongitude, 4),
    [PlanetField.OrbitalPeriod]: (galaxy, star, planet) =>
        trim(planet.orbitalPeriod, 4),
    [PlanetField.OrbitPhase]: (galaxy, star, planet) =>
        trim(planet.orbitPhase, 4),
    [PlanetField.Obliquity]: (galaxy, star, planet) =>
        trim(planet.obliquity, 4),
    [PlanetField.RotationPeriod]: (galaxy, star, planet) =>
        trim(planet.rotationPeriod, 4),
    [PlanetField.RotationPhase]: (galaxy, star, planet) =>
        trim(planet.rotationPhase, 4),
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
                output.push(
                    trim(nearestDistanceFrom(star.position, positions), 4),
                )
                break
            case StarField.DistanceFromFurthestX:
                output.push(
                    trim(furthestDistanceFrom(star.position, positions), 4),
                )
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
        output.push(trim(gases[type] ?? 0, 4))
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
        output.push(trim(gases[type] ?? 0, 4))
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
        })
        const data = generateExportData(result, indexes, exportAllStars)
        self.postMessage(data)
    })
}
