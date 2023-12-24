import { Component } from "solid-js"
import Button from "../components/Button"
import { useWorldGen } from "../worldgen"
import {
    ConditionType,
    OceanType,
    RuleType,
    SpectrType,
    VeinType,
} from "../enums"

const Find: Component = () => {
    const worldgen = useWorldGen()
    async function search() {
        worldgen().find({
            gameDesc: {
                starCount: 64,
                resourceMultiplier: 100,
            },
            range: [0, 99999999],
            rule: {
                type: RuleType.And,
                rules: [
                    {
                        type: RuleType.Spectr,
                        spectr: [SpectrType.O],
                    },
                    {
                        type: RuleType.OceanType,
                        oceanType: [OceanType.Water, OceanType.Sulfur],
                    },
                    {
                        type: RuleType.TidalLockCount,
                        condition: {
                            type: ConditionType.Gte,
                            value: 2,
                        },
                    },
                    {
                        type: RuleType.AverageVeinAmount,
                        vein: VeinType.Oil,
                        condition: {
                            type: ConditionType.Gte,
                            value: 2500000,
                        },
                    },
                    {
                        type: RuleType.GasCount,
                        condition: {
                            type: ConditionType.Gte,
                            value: 1,
                        },
                    },
                    {
                        type: RuleType.PlanetCount,
                        condition: {
                            type: ConditionType.Eq,
                            value: 6,
                        },
                    },
                    {
                        type: RuleType.Luminosity,
                        condition: {
                            type: ConditionType.Gte,
                            value: 15,
                        },
                    },
                ],
            },
            concurrency: 7,
            onProgress: (current, galaxys) => {
                console.log(current, galaxys)
            },
            onComplete: () => {
                console.log("done")
            },
            onInterrupt: () => {
                console.log("interrupt")
            },
        })
    }

    function stop() {
        worldgen().stop()
    }

    return (
        <>
            <Button onClick={search}>Click</Button>
            <Button onClick={stop}>Stop</Button>
        </>
    )
}

export default Find
