import styles from "./App.module.css"
import { generateGalaxy } from "./worldgen"

function App() {
    async function click() {
        console.time()
        for (let i = 0; i < 10000; ++i) {
            await generateGalaxy({ seed: i })
        }
        console.timeEnd()
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
