import { Component } from "solid-js"
import Select from "../components/Select"
import { hiveMaxDensityValues } from "../util"

const HiveMaxDensitySelector: Component<{
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
            options={hiveMaxDensityValues}
            getLabel={(x) => x + "x"}
            disabled={props.disabled}
        />
    )
}

export default HiveMaxDensitySelector
