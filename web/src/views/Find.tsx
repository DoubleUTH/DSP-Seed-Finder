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
import {
    getProfileSettings,
    saveToProfile,
    setProfileSettings,
} from "../profile"
import { constructRule } from "../util"

const Find: Component = () => {
    const worldgen = useWorldGen()
    async function search() {
        const rules: SimpleRule[] = [
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
                    value: 3,
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
        ]
        const existing = await getProfileSettings("test")
        const profile = existing || {
            id: "test",
            name: "Test",
            starCount: 64,
            resourceMultiplier: 100,
            start: 0,
            end: 99999999,
            current: 0,
            rules: rules.map((x) => [x]),
        }
        if (!existing) {
            await setProfileSettings(profile)
        }
        if (profile.current > profile.end) return
        worldgen().find({
            gameDesc: {
                starCount: profile.starCount,
                resourceMultiplier: profile.resourceMultiplier,
            },
            range: [profile.current, profile.end],
            rule: constructRule(profile.rules),
            concurrency: 7,
            onProgress: (current, galaxys) => {
                saveToProfile("test", current, galaxys)
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
