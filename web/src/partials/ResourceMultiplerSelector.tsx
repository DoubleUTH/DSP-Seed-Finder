import { Component } from "solid-js"
import Select from "../components/Select"

const ResourceMultiplierSelector: Component<{
    class?: string
    value: float
    onChange: (value: float) => void
    disabled?: boolean
}> = (props) => {
    return (
        <Select
            class={props.class}
            value={props.value}
            onChange={(v) => props.onChange(v)}
            options={[0.1, 0.5, 0.8, 1, 1.5, 2, 3, 5, 8, 100]}
            getLabel={(x) =>
                x === 100 ? "Infinite" : x <= 0.2 ? "Scarce" : x + "x"
            }
            disabled={props.disabled}
        />
    )
}

export default ResourceMultiplierSelector
