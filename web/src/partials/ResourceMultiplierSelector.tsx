import { Component } from "solid-js"
import Select from "../components/Select"
import { resourceMultipliers } from "../util"

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
            options={resourceMultipliers}
            getLabel={(x) =>
                x === 100 ? "Infinite" : x <= 0.2 ? "Scarce" : x + "x"
            }
            disabled={props.disabled}
        />
    )
}

export default ResourceMultiplierSelector
