import styles from "./Settings.module.css"
import { useStore } from "../store"
import Modal from "../components/Modal"
import { Component, createSignal } from "solid-js"
import Switch from "../components/Switch"
import NumberInput from "../components/NumberInput"
import Select from "../components/Select"

const Settings: Component = () => {
    const [store, setStore] = useStore()
    const [select, setSelect] = createSignal(false)

    function bind<K extends keyof Settings>(
        key: K,
    ): { value: Settings[K]; onChange: (v: Settings[K]) => void } {
        return {
            value: store.settings[key],
            onChange: (v) => setStore("settings", key as any, v),
        }
    }

    return (
        <Modal
            visible={store.modals.settings}
            onClose={() => setStore("modals", "settings", false)}
        >
            <div class={styles.title}>Settings</div>
            <div class={styles.row}>
                <div class={styles.field}>Dark Mode</div>
                <Switch {...bind("darkMode")} />
            </div>
            <div class={styles.row}>
                <div class={styles.field}>Native mode</div>
                <Switch {...bind("nativeMode")} />
            </div>
            <div class={styles.row}>
                <div class={styles.field}>Number of stars</div>
                <NumberInput
                    {...bind("starCount")}
                    min={32}
                    max={64}
                    step={1}
                    emptyValue={0}
                />
            </div>
            <div class={styles.row}>
                <div class={styles.field}>Resource Multiplier</div>
                <Select
                    {...bind("resourceMultiplier")}
                    getLabel={(x) =>
                        x === 100 ? "Infinite" : x <= 0.2 ? "Scarce" : x + "x"
                    }
                    options={[0.1, 0.5, 0.8, 1, 1.5, 2, 3, 5, 8, 100]}
                    isSelected={(t) => t === store.settings.resourceMultiplier}
                    focus={select()}
                    onFocusChange={(f) => setSelect(f)}
                />
            </div>
            <div class={styles.row}>
                <div class={styles.field}>Maximum concurrent threads</div>
                <NumberInput
                    {...bind("concurrency")}
                    emptyValue={0}
                    min={1}
                    max={9999}
                    step={1}
                />
            </div>
        </Modal>
    )
}

export default Settings
