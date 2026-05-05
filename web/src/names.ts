import { useLingui } from "#lingui"
import {
    GasType,
    StarType,
    RuleType,
    VeinType,
    ConditionType,
    SpectrType,
} from "./enums"

export function useGasTypeNames(): Record<GasType, () => string> {
    const { t } = useLingui()
    return {
        [GasType.None]: () => "",
        [GasType.Hydrogen]: () => t`Hydrogen`,
        [GasType.Deuterium]: () => t`Deuterium`,
        [GasType.Fireice]: () => t`Fire Ice`,
    }
}

export function useStarTypeNames(): Record<StarType, () => string> {
    const { t } = useLingui()
    return {
        [StarType.MainSeqStar]: () => t`Normal Star`,
        [StarType.GiantStar]: () => t`Giant Star`,
        [StarType.WhiteDwarf]: () => t`White Dwarf`,
        [StarType.BlackHole]: () => t`Black Hole`,
        [StarType.NeutronStar]: () => t`Neutron Star`,
    }
}

export function useRuleNames(): Record<RuleType, () => string> {
    const { t } = useLingui()
    return {
        [RuleType.None]: () => t`Select...`,
        [RuleType.And]: () => "",
        [RuleType.Or]: () => "",
        [RuleType.Birth]: () => t`Starting System`,
        [RuleType.StarType]: () => t`Type of star`,
        [RuleType.BirthDistance]: () => t`Distance from Start`,
        [RuleType.XDistance]: () => t`Distance from X Star`,
        [RuleType.SpectrDistance]: () => t`Distance from Other Stars`,
        [RuleType.Luminosity]: () => t`Luminosity`,
        [RuleType.Spectr]: () => t`Spectral Class`,
        [RuleType.DysonRadius]: () => t`Max Dyson Sphere Radius`,
        [RuleType.PlanetCount]: () => t`Planet Count`,
        [RuleType.SatelliteCount]: () => t`Satellite Count`,
        [RuleType.TidalLockCount]: () => t`Tidally Locked Planet Count`,
        [RuleType.ThemeId]: () => t`Planet Themes`,
        [RuleType.GasCount]: () => t`Gas/Ice Giant Count`,
        [RuleType.OceanType]: () => t`Ocean`,
        [RuleType.GasRate]: () => t`Gas Rate`,
        [RuleType.AverageVeinAmount]: () => t`Vein Amount`,
        [RuleType.PlanetInDysonCount]: () => t`Planets in Dyson Sphere`,
        [RuleType.HiveCount]: () => t`Hive Count`,
    }
}

export function useVeinNames(): Record<VeinType, () => string> {
    const { t } = useLingui()
    return {
        [VeinType.None]: () => "",
        [VeinType.Iron]: () => t`Iron Ore`,
        [VeinType.Copper]: () => t`Copper Ore`,
        [VeinType.Silicium]: () => t`Silicon Ore`,
        [VeinType.Titanium]: () => t`Titanium Ore`,
        [VeinType.Stone]: () => t`Stone`,
        [VeinType.Coal]: () => t`Coal`,
        [VeinType.Oil]: () => t`Crude Oil`,
        [VeinType.Fireice]: () => t`Fire Ice`,
        [VeinType.Diamond]: () => t`Kimberlite Ore`,
        [VeinType.Fractal]: () => t`Fractal Silicon`,
        [VeinType.Crysrub]: () => t`Organic Crystal`,
        [VeinType.Grat]: () => t`Grating Crystal`,
        [VeinType.Bamboo]: () => t`Stalagmite Crystal`,
        [VeinType.Mag]: () => t`Unipolar Magnet`,
    }
}

export function usePlanetTypeNames(): Record<number, () => string> {
    const { t } = useLingui()
    return {
        1: () => t`Mariterra`,
        2: () => t`Gas Giant`,
        3: () => t`Gas Giant`,
        4: () => t`Ice Giant`,
        5: () => t`Ice Giant`,
        6: () => t`Scorchedia`,
        7: () => t`Geloterra`,
        8: () => t`Tropicana`,
        9: () => t`Lava`,
        10: () => t`Glacieon`,
        11: () => t`Desolus`,
        12: () => t`Gobi`,
        13: () => t`Sulfuria`,
        14: () => t`Crimsonis`,
        15: () => t`Prairiea`,
        16: () => t`Aquatica`,
        17: () => t`Halitum`,
        18: () => t`Sakura Ocean`,
        19: () => t`Cyclonius`,
        20: () => t`Maroonfrost`,
        21: () => t`Gas Giant`,
        22: () => t`Savanna`,
        23: () => t`Onyxtopia`,
        24: () => t`Icefrostia`,
        25: () => t`Pandora Swamp`,
    }
}

export function useConditionTypeNames(): Record<ConditionType, () => string> {
    const { t } = useLingui()
    return {
        [ConditionType.Eq]: () => t`exactly`,
        [ConditionType.Neq]: () => t`not equal to`,
        [ConditionType.Gt]: () => t`greater than`,
        [ConditionType.Gte]: () => t`at least`,
        [ConditionType.Lt]: () => t`less than`,
        [ConditionType.Lte]: () => t`at most`,
    }
}

export function useStarTypeFullName(): (star: Star) => string {
    const { t } = useLingui()
    return (star) => {
        if (star.type === StarType.GiantStar) {
            switch (star.spectr) {
                case SpectrType.M:
                case SpectrType.K:
                    return t`Red Giant`
                case SpectrType.G:
                case SpectrType.F:
                    return t`Yellow Giant`
                case SpectrType.A:
                    return t`White Giant`
                default:
                    return t`Blue Giant`
            }
        } else if (star.type === StarType.WhiteDwarf) {
            return t`White Dwarf`
        } else if (star.type === StarType.NeutronStar) {
            return t`Neutron Star`
        } else if (star.type === StarType.BlackHole) {
            return t`Black Hole`
        } else {
            return t`${star.spectr} type Star`
        }
    }
}
