import styles from "./App.module.css"
import { WorldGenImpl } from "worldgen-impl"
import { ConditionType, RuleType, VeinType } from "./enums"

function App() {
    async function click() {
        const worldgen: WorldGen = new WorldGenImpl()
        console.log(await worldgen.generate({ seed: 0 }))
    }
    async function click2() {
        const worldgen: WorldGen = new WorldGenImpl()
        const g = worldgen.find({}, [0, 1000], {
            type: RuleType.AverageVeinPatch,
            vein: VeinType.Mag,
            condition: {
                type: ConditionType.Gt,
                value: 1,
            },
        })
        for await (const r of g) {
            console.log(r)
        }
    }
    return (
        <div class={styles.app}>
            <button type="button" onClick={click}>
                Click
            </button>
            <button type="button" onClick={click2}>
                Click2
            </button>
        </div>
    )
}

export default App
