import styles from "./App.module.css"
import { WorldGenImpl } from "worldgen-impl"

function App() {
    async function click() {
        const worldgen: WorldGen = new WorldGenImpl()
        await Promise.all(
            Array.from({ length: 1000 }).map((_, seed) =>
                worldgen.generate({ seed }),
            ),
        )
        worldgen.destroy()
    }
    return (
        <div class={styles.app}>
            <button type="button" onClick={click}>
                Click
            </button>
        </div>
    )
}

export default App
